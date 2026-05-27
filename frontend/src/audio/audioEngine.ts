/**
 * Web Audio API Guitar Engine
 *
 * When running outside Tauri (browser dev mode), this engine provides real
 * guitar signal processing so the app is not just a UI shell.
 *
 * Signal chain: Input → Boost → Amp (waveshaper + EQ) → Cab (convolver) →
 *              Delay → Reverb → Output
 */

import type { ChainSnapshot } from '../types/tauri';

export interface WebAudioState {
  running: boolean;
  sampleRate: number;
  bufferSize: number;
}

class AudioEngine {
  private ctx: AudioContext | null = null;
  private stream: MediaStream | null = null;
  private source: AudioNode | null = null;
  private testOsc: OscillatorNode | null = null;

  // Signal chain nodes
  private inputGain: GainNode | null = null;
  private boostGain: GainNode | null = null;
  private ampShaper: WaveShaperNode | null = null;
  private ampBass: BiquadFilterNode | null = null;
  private ampMid: BiquadFilterNode | null = null;
  private ampTreble: BiquadFilterNode | null = null;
  private ampMaster: GainNode | null = null;
  private cabConvolver: ConvolverNode | null = null;
  private cabLowCut: BiquadFilterNode | null = null;
  private cabHighCut: BiquadFilterNode | null = null;
  private delayNode: DelayNode | null = null;
  private delayFeedback: GainNode | null = null;
  private delayMix: GainNode | null = null;
  private reverbConvolver: ConvolverNode | null = null;
  private reverbMix: GainNode | null = null;
  private outputGain: GainNode | null = null;

  // Analysis
  private analyser: AnalyserNode | null = null;

  // State
  private state: WebAudioState = { running: false, sampleRate: 48000, bufferSize: 256 };
  private slotEnabled = new Map<string, boolean>();
  private slotParams = new Map<string, Record<string, number>>();

  // Listeners
  private levelCallbacks: ((levels: number[]) => void)[] = [];
  private levelInterval: ReturnType<typeof setInterval> | null = null;

  /** Start the engine: request mic permission and build the graph. */
  async start(): Promise<void> {
    if (this.ctx?.state === 'running') return;

    try {
      this.stream = await navigator.mediaDevices.getUserMedia({
        audio: {
          echoCancellation: false,
          noiseSuppression: false,
          autoGainControl: false,
          sampleRate: 48000,
        },
      });
    } catch (err) {
      console.warn('[WebAudio] Could not get microphone access:', err);
      // Fallback: create offline-ish context for testing (oscillator)
      this.createTestOscillator();
      return;
    }

    this.ctx = new AudioContext({ sampleRate: 48000, latencyHint: 'interactive' });
    this.source = this.ctx.createMediaStreamSource(this.stream);
    this.state.sampleRate = this.ctx.sampleRate;
    this.buildGraph();
    this.source.connect(this.inputGain!);
    await this.ctx.resume();
    this.state.running = true;
    this.startLevelPolling();
  }

  /** Stop and release all resources. */
  async stop(): Promise<void> {
    this.stopLevelPolling();
    if (this.levelInterval) {
      clearInterval(this.levelInterval);
      this.levelInterval = null;
    }
    if (this.testOsc) {
      try { this.testOsc.stop(); } catch { /* no-op */ }
      this.testOsc = null;
    }
    if (this.source) {
      try { this.source.disconnect(); } catch { /* no-op */ }
      this.source = null;
    }
    if (this.stream) {
      this.stream.getTracks().forEach((t) => t.stop());
      this.stream = null;
    }
    if (this.ctx) {
      try { await this.ctx.close(); } catch { /* no-op */ }
      this.ctx = null;
    }
    this.state.running = false;
  }

  /** Rebuild the graph from a ChainSnapshot. */
  applyChain(chain: ChainSnapshot): void {
    for (const slot of chain.slots) {
      this.slotEnabled.set(slot.id, slot.enabled);
      this.slotParams.set(slot.id, { ...slot.parameters });
    }
    this.updateGraphFromState();
  }

  /** Update a single parameter. */
  setParameter(slotId: string, paramId: string, value: number): void {
    const params = this.slotParams.get(slotId) ?? {};
    params[paramId] = value;
    this.slotParams.set(slotId, params);
    this.updateGraphFromState();
  }

  /** Toggle a slot on/off. */
  toggleSlot(slotId: string, enabled: boolean): void {
    this.slotEnabled.set(slotId, enabled);
    this.updateGraphFromState();
  }

  /** Get current state. */
  getState(): WebAudioState {
    return { ...this.state };
  }

  /** Subscribe to level updates (returns unsubscribe fn). */
  onLevels(cb: (levels: number[]) => void): () => void {
    this.levelCallbacks.push(cb);
    return () => {
      this.levelCallbacks = this.levelCallbacks.filter((c) => c !== cb);
    };
  }

  /** Read instantaneous RMS levels per slot. */
  getLevels(): number[] {
    if (!this.analyser || !this.ctx) return [];
    const data = new Float32Array(this.analyser.frequencyBinCount);
    this.analyser.getFloatTimeDomainData(data);
    // Compute RMS
    let sum = 0;
    for (let i = 0; i < data.length; i++) {
      sum += data[i] * data[i];
    }
    const rms = Math.sqrt(sum / data.length);
    // Distribute across slots for visual effect
    const base = Math.min(rms * 4, 1);
    return Array.from({ length: 8 }, (_, i) => {
      const id = this.getSlotIdByIndex(i);
      const enabled = id ? (this.slotEnabled.get(id) ?? true) : true;
      if (!enabled) return 0;
      // Add slight variation per slot
      return Math.min(1, Math.max(0, base * (0.6 + 0.4 * Math.sin(Date.now() / 200 + i))));
    });
  }

  /** Expose the analyser for the visualizer to read real audio data. */
  getAnalyser(): AnalyserNode | null {
    return this.analyser;
  }

  // ── Private ──

  private getSlotIdByIndex(index: number): string | null {
    const ids = ['input', 'boost', 'amp', 'cab', 'bass-amp', 'delay', 'reverb', 'output'];
    return ids[index] ?? null;
  }

  private createTestOscillator(): void {
    // When no mic, create an oscillator so the UI still has levels to show
    this.ctx = new AudioContext({ sampleRate: 48000 });
    const osc = this.ctx.createOscillator();
    this.testOsc = osc;
    osc.type = 'sawtooth';
    osc.frequency.value = 110; // A2
    const oscGain = this.ctx.createGain();
    oscGain.gain.value = 0.15;
    osc.connect(oscGain);
    this.source = oscGain;
    this.buildGraph();
    oscGain.connect(this.inputGain!);
    osc.start();
    this.ctx.resume();
    this.state.running = true;
    this.startLevelPolling();
  }

  private buildGraph(): void {
    if (!this.ctx) return;
    const c = this.ctx;

    // Input
    this.inputGain = c.createGain();
    this.inputGain.gain.value = 0.8;

    // Boost
    this.boostGain = c.createGain();
    this.boostGain.gain.value = 1.0;

    // Amp: waveshaper + 3-band EQ + master
    this.ampShaper = c.createWaveShaper();
    this.ampShaper.curve = this.makeDistortionCurve(0) as any;
    this.ampShaper.oversample = '4x';

    this.ampBass = c.createBiquadFilter();
    this.ampBass.type = 'lowshelf';
    this.ampBass.frequency.value = 250;
    this.ampBass.gain.value = 0;

    this.ampMid = c.createBiquadFilter();
    this.ampMid.type = 'peaking';
    this.ampMid.frequency.value = 800;
    this.ampMid.Q.value = 1.0;
    this.ampMid.gain.value = 0;

    this.ampTreble = c.createBiquadFilter();
    this.ampTreble.type = 'highshelf';
    this.ampTreble.frequency.value = 3000;
    this.ampTreble.gain.value = 0;

    this.ampMaster = c.createGain();
    this.ampMaster.gain.value = 0.7;

    // Cab: convolver + cuts
    this.cabConvolver = c.createConvolver();
    this.cabConvolver.buffer = this.generateCabImpulse(c, 1024);
    this.cabLowCut = c.createBiquadFilter();
    this.cabLowCut.type = 'highpass';
    this.cabLowCut.frequency.value = 80;
    this.cabHighCut = c.createBiquadFilter();
    this.cabHighCut.type = 'lowpass';
    this.cabHighCut.frequency.value = 8000;

    // Delay
    this.delayNode = c.createDelay(5.0);
    this.delayNode.delayTime.value = 0.3;
    this.delayFeedback = c.createGain();
    this.delayFeedback.gain.value = 0.0;
    this.delayMix = c.createGain();
    this.delayMix.gain.value = 0.0;

    // Reverb
    this.reverbConvolver = c.createConvolver();
    this.reverbConvolver.buffer = this.generateReverbImpulse(c, 2.0, 2.0);
    this.reverbMix = c.createGain();
    this.reverbMix.gain.value = 0.0;

    // Output
    this.outputGain = c.createGain();
    this.outputGain.gain.value = 0.8;

    // Analyser
    this.analyser = c.createAnalyser();
    this.analyser.fftSize = 256;
    this.analyser.smoothingTimeConstant = 0.8;

    // Wiring
    // input -> boost -> amp(shaper->bass->mid->treble->master) -> cab(conv->low->high) -> delay -> reverb -> output -> analyser -> dest
    this.inputGain.connect(this.boostGain!);
    this.boostGain!.connect(this.ampShaper!);
    this.ampShaper!.connect(this.ampBass!);
    this.ampBass!.connect(this.ampMid!);
    this.ampMid!.connect(this.ampTreble!);
    this.ampTreble!.connect(this.ampMaster!);
    this.ampMaster!.connect(this.cabConvolver!);
    this.cabConvolver!.connect(this.cabLowCut!);
    this.cabLowCut!.connect(this.cabHighCut!);
    this.cabHighCut!.connect(this.delayNode!);
    this.delayNode!.connect(this.delayFeedback!);
    this.delayFeedback!.connect(this.delayNode!);
    this.delayNode!.connect(this.delayMix!);
    this.cabHighCut!.connect(this.reverbConvolver!);
    this.reverbConvolver!.connect(this.reverbMix!);

    // Mix back to main path
    const dryGain = c.createGain();
    dryGain.gain.value = 1.0;
    this.cabHighCut!.connect(dryGain);

    const mixGain = c.createGain();
    mixGain.gain.value = 1.0;
    dryGain.connect(mixGain);
    this.delayMix!.connect(mixGain);
    this.reverbMix!.connect(mixGain);
    mixGain.connect(this.outputGain!);
    this.outputGain!.connect(this.analyser!);
    this.analyser!.connect(c.destination);
  }

  private updateGraphFromState(): void {
    if (!this.ctx) return;

    // Boost
    const boostParams = this.slotParams.get('boost') ?? {};
    const boostGain = boostParams.gain ?? 0.75;
    if (this.boostGain) {
      const enabled = this.slotEnabled.get('boost') ?? true;
      const target = enabled ? 1.0 + boostGain * 2.0 : 1.0;
      this.smoothSet(this.boostGain.gain, target, 0.02);
    }

    // Amp
    const ampParams = this.slotParams.get('amp') ?? {};
    const ampEnabled = this.slotEnabled.get('amp') ?? true;
    if (this.ampShaper) {
      const drive = ampParams.drive ?? 0.5;
      this.ampShaper.curve = this.makeDistortionCurve(ampEnabled ? drive : 0) as any;
    }
    if (this.ampBass) {
      const bass = ampParams.bass ?? 0.5;
      this.smoothSet(this.ampBass.gain, ampEnabled ? (bass - 0.5) * 24 : 0, 0.05);
    }
    if (this.ampMid) {
      const mid = ampParams.mid ?? 0.5;
      this.smoothSet(this.ampMid.gain, ampEnabled ? (mid - 0.5) * 24 : 0, 0.05);
    }
    if (this.ampTreble) {
      const treble = ampParams.treble ?? 0.5;
      this.smoothSet(this.ampTreble.gain, ampEnabled ? (treble - 0.5) * 24 : 0, 0.05);
    }
    if (this.ampMaster) {
      const master = ampParams.master ?? 0.7;
      this.smoothSet(this.ampMaster.gain, ampEnabled ? master : 0, 0.05);
    }

    // Cab
    const cabParams = this.slotParams.get('cab') ?? {};
    const cabEnabled = this.slotEnabled.get('cab') ?? true;
    if (this.cabLowCut) {
      const lowCut = cabParams.low_cut ?? 0.0;
      this.cabLowCut.frequency.value = cabEnabled ? 20 + lowCut * 230 : 20;
    }
    if (this.cabHighCut) {
      const highCut = cabParams.high_cut ?? 0.6;
      this.cabHighCut.frequency.value = cabEnabled ? 8000 - highCut * 6000 : 20000;
    }
    if (this.cabConvolver) {
      // Can't easily bypass convolver without rebuilding, so scale output
      // We'll just let it run; the cab EQ handles most of the shaping
    }

    // Delay
    const delayParams = this.slotParams.get('delay') ?? {};
    const delayEnabled = this.slotEnabled.get('delay') ?? false;
    if (this.delayNode) {
      const time = delayParams.time ?? 0.3;
      this.delayNode.delayTime.value = 0.02 + time * 1.98;
    }
    if (this.delayFeedback) {
      const feedback = delayParams.feedback ?? 0.4;
      this.delayFeedback.gain.value = delayEnabled ? feedback * 0.95 : 0;
    }
    if (this.delayMix) {
      const mix = delayParams.mix ?? 0.3;
      this.delayMix.gain.value = delayEnabled ? mix : 0;
    }

    // Reverb
    const reverbParams = this.slotParams.get('reverb') ?? {};
    const reverbEnabled = this.slotEnabled.get('reverb') ?? true;
    if (this.reverbMix) {
      const mix = reverbParams.mix ?? 0.3;
      this.reverbMix.gain.value = reverbEnabled ? mix * 0.6 : 0;
    }
    if (this.reverbConvolver && this.ctx) {
      const size = reverbParams.size ?? 0.5;
      const damping = reverbParams.damping ?? 0.5;
      // Regenerate reverb IR on significant size changes (debounced in real impl)
      // For simplicity we skip regeneration here and use fixed IR
      void size;
      void damping;
    }

    // Output
    const outputParams = this.slotParams.get('output') ?? {};
    if (this.outputGain) {
      const vol = outputParams.volume ?? 0.8;
      this.smoothSet(this.outputGain.gain, vol, 0.05);
    }
  }

  private smoothSet(param: AudioParam, target: number, timeConstant: number): void {
    if (!this.ctx) return;
    param.setTargetAtTime(Math.max(0, target), this.ctx.currentTime, timeConstant);
  }

  private makeDistortionCurve(drive: number): Float32Array {
    const samples = 44100;
    const curve = new Float32Array(samples);
    const deg = Math.PI / 180;
    const amount = drive * 400; // 0..400
    for (let i = 0; i < samples; i++) {
      const x = (i * 2) / samples - 1;
      // Asymmetric tube-like distortion
      curve[i] = (3 + amount) * x * 20 * deg / (Math.PI + amount * Math.abs(x));
      // Blend with clean based on drive
      const blend = Math.min(1, drive * 2);
      curve[i] = curve[i] * blend + x * (1 - blend);
      // Soft clip
      curve[i] = Math.tanh(curve[i]);
    }
    return curve;
  }

  private generateCabImpulse(ctx: AudioContext, length: number): AudioBuffer {
    const sampleRate = ctx.sampleRate;
    const buffer = ctx.createBuffer(1, length, sampleRate);
    const data = buffer.getChannelData(0);
    for (let i = 0; i < length; i++) {
      const t = i / sampleRate;
      // Decaying noise + resonances for speaker character
      const decay = Math.exp(-t * 20);
      const resonance1 = Math.sin(2 * Math.PI * 120 * t) * 0.3;
      const resonance2 = Math.sin(2 * Math.PI * 2500 * t) * 0.1;
      data[i] = (Math.random() * 2 - 1) * decay * 0.5 + resonance1 * decay + resonance2 * decay;
    }
    return buffer;
  }

  private generateReverbImpulse(ctx: AudioContext, duration: number, decay: number): AudioBuffer {
    const sampleRate = ctx.sampleRate;
    const samples = Math.floor(sampleRate * duration);
    const buffer = ctx.createBuffer(2, samples, sampleRate);
    for (let ch = 0; ch < 2; ch++) {
        const data = buffer.getChannelData(ch);
      for (let i = 0; i < samples; i++) {
        const t = i / sampleRate;
        const env = Math.exp(-t * (1 / (decay + 0.1)));
        data[i] = (Math.random() * 2 - 1) * env * (1 - t / duration);
      }
    }
    return buffer;
  }

  private startLevelPolling(): void {
    this.stopLevelPolling();
    this.levelInterval = setInterval(() => {
      const levels = this.getLevels();
      for (const cb of this.levelCallbacks) {
        try { cb(levels); } catch { /* no-op */ }
      }
    }, 50);
  }

  private stopLevelPolling(): void {
    if (this.levelInterval) {
      clearInterval(this.levelInterval);
      this.levelInterval = null;
    }
  }
}

export const webAudioEngine = new AudioEngine();
