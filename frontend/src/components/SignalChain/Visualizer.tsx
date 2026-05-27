import { useRef, useEffect } from 'react';
import { useEngineStore } from '../../stores/engineStore';
import { webAudioEngine } from '../../audio/audioEngine';

export function Visualizer() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animRef = useRef<number>(0);
  const status = useEngineStore((s) => s.status);
  const isRunning = status.running;
  const timeRef = useRef(0);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    let accent = '#00f0ff';
    let glow = '#00f0ff';
    let bgSurface = '#0e0e14';
    let border = '#1e1e2e';
    let textMuted = '#606070';

    const updateColors = () => {
      const style = getComputedStyle(document.documentElement);
      accent = style.getPropertyValue('--accent').trim() || accent;
      glow = style.getPropertyValue('--glow').trim() || glow;
      bgSurface = style.getPropertyValue('--bg-surface').trim() || bgSurface;
      border = style.getPropertyValue('--border').trim() || border;
      textMuted = style.getPropertyValue('--text-muted').trim() || textMuted;
    };
    updateColors();

    const resize = () => {
      const rect = canvas.parentElement?.getBoundingClientRect();
      if (rect) {
        const dpr = window.devicePixelRatio || 1;
        canvas.width = rect.width * dpr;
        canvas.height = rect.height * dpr;
        canvas.style.width = rect.width + 'px';
        canvas.style.height = rect.height + 'px';
        const ctx = canvas.getContext('2d');
        if (ctx) ctx.scale(dpr, dpr);
      }
    };
    resize();
    window.addEventListener('resize', resize);

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const draw = () => {
      const rect = canvas.parentElement?.getBoundingClientRect();
      const w = rect?.width ?? canvas.width;
      const h = rect?.height ?? canvas.height;
      timeRef.current += 0.016;
      const t = timeRef.current;

      ctx.fillStyle = bgSurface;
      ctx.fillRect(0, 0, w, h);

      // Grid lines
      ctx.strokeStyle = border;
      ctx.lineWidth = 0.5;
      ctx.beginPath();
      ctx.moveTo(0, h / 2);
      ctx.lineTo(w, h / 2);
      ctx.stroke();

      // Read real audio data when running
      let waveform: Float32Array | null = null;
      let freqData: Uint8Array | null = null;

      if (isRunning) {
        const analyser = webAudioEngine.getAnalyser();
        if (analyser) {
          waveform = new Float32Array(analyser.frequencyBinCount);
          analyser.getFloatTimeDomainData(waveform as any);
          freqData = new Uint8Array(analyser.frequencyBinCount);
          analyser.getByteFrequencyData(freqData as any);
        }
      }

      // Draw waveform
      ctx.strokeStyle = accent;
      ctx.lineWidth = 2;
      ctx.shadowColor = glow;
      ctx.shadowBlur = isRunning ? 12 : 6;
      ctx.beginPath();

      const points = 300;
      for (let i = 0; i < points; i++) {
        const x = (i / (points - 1)) * w;
        let sample = 0;
        if (waveform && waveform.length > 0) {
          const idx = Math.floor((i / points) * waveform.length);
          sample = waveform[idx];
        } else {
          // Demo animation when stopped
          const freq = 2 + Math.sin(t * 0.3) * 1.5;
          sample = Math.sin(i * 0.05 * freq + t * 3) * 0.15;
          sample += Math.sin(i * 0.12 * freq + t * 5) * 0.08;
          sample += Math.sin(i * 0.03 + t * 1.5) * 0.05;
        }
        const y = h / 2 + sample * h * 0.4;
        if (i === 0) ctx.moveTo(x, y);
        else ctx.lineTo(x, y);
      }
      ctx.stroke();
      ctx.shadowBlur = 0;

      // Draw FFT-style bars
      const barCount = 64;
      const barWidth = w / barCount - 1;
      const baseY = h - 4;

      for (let i = 0; i < barCount; i++) {
        let height = 0;
        if (freqData && freqData.length > 0) {
          const binsPerBar = Math.floor(freqData.length / barCount);
          let sum = 0;
          for (let j = 0; j < binsPerBar; j++) {
            sum += freqData[i * binsPerBar + j];
          }
          height = Math.max(2, (sum / binsPerBar / 255) * (h * 0.35));
        } else {
          // Demo bars
          const demoFreq = (i / barCount) * Math.PI;
          height = Math.max(2, (Math.sin(demoFreq + t * 2) * 0.5 + 0.5) * (h * 0.15) * (1 - i / barCount * 0.3));
        }
        const x = i * (w / barCount);
        const y = baseY - height;

        const gradient = ctx.createLinearGradient(0, baseY, 0, y);
        gradient.addColorStop(0, accent + '20');
        gradient.addColorStop(1, glow + (isRunning ? 'cc' : '66'));

        ctx.fillStyle = gradient;
        ctx.fillRect(x + 0.5, y, barWidth - 0.5, height);
      }

      // Zero-dBFS marker
      ctx.strokeStyle = '#ff2a6d44';
      ctx.lineWidth = 1;
      ctx.setLineDash([4, 4]);
      ctx.beginPath();
      ctx.moveTo(w * 0.92, 0);
      ctx.lineTo(w * 0.92, h);
      ctx.stroke();
      ctx.setLineDash([]);

      animRef.current = requestAnimationFrame(draw);
    };

    animRef.current = requestAnimationFrame(draw);

    return () => {
      window.removeEventListener('resize', resize);
      cancelAnimationFrame(animRef.current);
    };
  }, [isRunning]);

  return (
    <div className="w-full h-36 rounded-xl overflow-hidden border border-[var(--border)] bg-[var(--bg-surface)] relative">
      <div className="absolute top-2 left-3 z-10 flex items-center gap-2">
        <span className={`w-1.5 h-1.5 rounded-full ${isRunning ? 'bg-[var(--success)] animate-pulse' : 'bg-[var(--warning)]'}`} />
        <span className="font-mono-data text-[10px] text-[var(--text-muted)] tracking-wider uppercase">
          {isRunning ? 'LIVE INPUT' : 'DEMO MODE'}
        </span>
      </div>
      <canvas
        ref={canvasRef}
        className="visualizer w-full h-full block"
      />
    </div>
  );
}
