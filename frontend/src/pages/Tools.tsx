import { useState, useEffect, useRef, useCallback } from 'react';
import { useEngineStore } from '../stores/engineStore';
import {
  getTunerInfo,
  setParameter,
  getLooperState,
  triggerLooperMode,
  looperClear,
} from '../lib/tauri';

const NOTE_NAMES = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'];

function freqToNote(freq: number): { note: string; cents: number; octave: number } {
  if (freq <= 0) return { note: '—', cents: 0, octave: 0 };
  const A4 = 440;
  const semitones = 12 * Math.log2(freq / A4);
  const noteIndex = Math.round(semitones) + 9;
  const octave = Math.floor((noteIndex + 48) / 12);
  const wrappedIndex = ((noteIndex % 12) + 12) % 12;
  const cents = (semitones - Math.round(semitones)) * 100;
  return { note: NOTE_NAMES[wrappedIndex], cents, octave };
}

// ─── Tuner ───
function TunerPanel() {
  const [note, setNote] = useState({ note: '—', cents: 0, octave: 0 });
  const [detectedFreq, setDetectedFreq] = useState(0);
  const [confidence, setConfidence] = useState(0);
  const isTauri = useEngineStore((s) => s.isTauri);

  useEffect(() => {
    // Poll tuner info from backend if in Tauri mode; otherwise simulate
    let cancelled = false;
    const poll = async () => {
      if (isTauri) {
        try {
          const info = await getTunerInfo();
          if (cancelled) return;
          const n = freqToNote(info.frequency);
          setNote(n);
          setDetectedFreq(info.frequency);
          setConfidence(info.confidence);
        } catch {
          /* ignore */
        }
      } else {
        // Browser dev mode: simulate wandering pitch
        const t = Date.now() / 500;
        const baseFreq = 220 + Math.sin(t * 0.5) * 30 + Math.sin(t * 1.3) * 15;
        const n = freqToNote(baseFreq);
        setNote(n);
        setDetectedFreq(baseFreq);
        setConfidence(0.85);
      }
    };
    const id = setInterval(poll, 100);
    return () => { cancelled = true; clearInterval(id); };
  }, [isTauri]);

  const isInTune = Math.abs(note.cents) < 5 && confidence > 0.3;
  const needlePos = Math.max(-50, Math.min(50, note.cents));

  return (
    <div className="border border-[var(--border)] rounded-xl p-5 bg-[var(--bg-surface)]">
      <div className="flex items-center justify-between mb-4">
        <h3 className="font-display text-sm font-bold text-[var(--text-h)] tracking-wider">TUNER</h3>
        <span className="font-mono-data text-[10px] text-[var(--text-muted)]">{detectedFreq.toFixed(1)} Hz</span>
      </div>

      <div className="flex items-center justify-center mb-4">
        <div className={`text-6xl font-black font-display tracking-wider transition-colors duration-200 ${isInTune ? 'text-[var(--success)] neon-text' : 'text-[var(--text-h)]'}`}>
          {note.note}<span className="text-2xl ml-1">{note.octave}</span>
        </div>
      </div>

      <div className="relative h-2 bg-[var(--bg-elevated)] rounded-full mb-1 overflow-hidden">
        <div className="absolute top-0 left-1/2 w-px h-full bg-[var(--success)]" />
        <div
          className={`absolute top-0 w-1 h-full rounded-full transition-all duration-150 ${isInTune ? 'bg-[var(--success)]' : 'bg-[var(--warning)]'}`}
          style={{ left: `calc(50% + ${needlePos}%)` }}
        />
      </div>
      <div className="flex justify-between text-[9px] text-[var(--text-muted)] font-mono-data">
        <span>-50¢</span>
        <span className={isInTune ? 'text-[var(--success)]' : ''}>{note.cents.toFixed(0)}¢</span>
        <span>+50¢</span>
      </div>

      <div className="mt-3 flex items-center justify-center gap-2">
        <span className={`w-2 h-2 rounded-full ${isInTune ? 'bg-[var(--success)] animate-pulse' : 'bg-[var(--text-muted)]'}`} />
        <span className={`text-xs font-medium ${isInTune ? 'text-[var(--success)]' : 'text-[var(--text-muted)]'}`}>
          {isInTune ? 'IN TUNE' : 'TUNE UP'}
        </span>
      </div>
    </div>
  );
}

// ─── Metronome ───
function MetronomePanel() {
  const [bpm, setBpm] = useState(120);
  const [isPlaying, setIsPlaying] = useState(false);
  const [beat, setBeat] = useState(0);
  const isTauri = useEngineStore((s) => s.isTauri);
  const intervalRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const updateBackend = useCallback(async (newBpm: number, running: boolean) => {
    if (isTauri) {
      await setParameter('bpm', newBpm);
      await setParameter('running', running ? 1.0 : 0.0);
    }
  }, [isTauri]);

  const start = useCallback(() => {
    if (intervalRef.current) clearInterval(intervalRef.current);
    const interval = (60 / bpm) * 1000;
    let currentBeat = 0;
    intervalRef.current = setInterval(() => {
      currentBeat = (currentBeat + 1) % 4;
      setBeat(currentBeat);
    }, interval);
    setIsPlaying(true);
    updateBackend(bpm, true);
  }, [bpm, updateBackend]);

  const stop = useCallback(() => {
    if (intervalRef.current) {
      clearInterval(intervalRef.current);
      intervalRef.current = null;
    }
    setIsPlaying(false);
    setBeat(0);
    updateBackend(bpm, false);
  }, [bpm, updateBackend]);

  const toggle = useCallback(() => {
    if (isPlaying) stop();
    else start();
  }, [isPlaying, start, stop]);

  useEffect(() => {
    if (!isPlaying) return;
    if (intervalRef.current) clearInterval(intervalRef.current);
    const interval = (60 / bpm) * 1000;
    let currentBeat = 0;
    intervalRef.current = setInterval(() => {
      currentBeat = (currentBeat + 1) % 4;
      setBeat(currentBeat);
    }, interval);
    updateBackend(bpm, true);
    return () => { if (intervalRef.current) clearInterval(intervalRef.current); };
  }, [bpm]); // eslint-disable-line react-hooks/exhaustive-deps

  return (
    <div className="border border-[var(--border)] rounded-xl p-5 bg-[var(--bg-surface)]">
      <h3 className="font-display text-sm font-bold text-[var(--text-h)] tracking-wider mb-4">METRONOME</h3>

      <div className="flex items-center justify-center mb-4">
        <div className="font-mono-data text-5xl font-bold text-[var(--accent)] tracking-wider">
          {bpm}
        </div>
        <span className="text-xs text-[var(--text-muted)] ml-2 mt-4">BPM</span>
      </div>

      <input
        type="range"
        min={40}
        max={240}
        step={1}
        value={bpm}
        onChange={(e) => setBpm(parseInt(e.target.value))}
        className="w-full mb-4"
      />

      <div className="flex items-center gap-2 mb-4">
        {[40, 60, 80, 100, 120, 140, 160, 180, 200].map((preset) => (
          <button
            key={preset}
            onClick={() => setBpm(preset)}
            className={`flex-1 py-1 rounded text-[10px] font-mono-data transition-colors cursor-pointer ${
              bpm === preset
                ? 'bg-[var(--accent)] text-[#07070a] font-bold'
                : 'bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-[var(--text)]'
            }`}
          >
            {preset}
          </button>
        ))}
      </div>

      <button
        onClick={toggle}
        className={`w-full py-2.5 rounded-lg text-sm font-bold tracking-wider transition-all cursor-pointer ${
          isPlaying
            ? 'bg-[var(--danger)]/20 text-[var(--danger)] border border-[var(--danger)]/30 hover:bg-[var(--danger)]/30'
            : 'bg-[var(--accent)] text-[#07070a] neon-button'
        }`}
      >
        {isPlaying ? 'STOP' : 'START'}
      </button>

      <div className="flex items-center justify-center gap-3 mt-4">
        {[0, 1, 2, 3].map((i) => (
          <div
            key={i}
            className={`w-3 h-3 rounded-full transition-all duration-100 ${
              isPlaying && beat === i
                ? 'bg-[var(--accent)] scale-125 neon-border'
                : 'bg-[var(--bg-elevated)] border border-[var(--border)]'
            }`}
          />
        ))}
      </div>
    </div>
  );
}

// ─── Looper ───
function LooperPanel() {
  const [mode, setMode] = useState('idle');
  const [loopLength, setLoopLength] = useState(0);
  const isTauri = useEngineStore((s) => s.isTauri);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const refresh = useCallback(async () => {
    try {
      const state = await getLooperState();
      setMode(state.mode);
      setLoopLength(state.loop_time_seconds);
    } catch {
      /* ignore */
    }
  }, []);

  const startRecording = useCallback(async () => {
    if (isTauri) await triggerLooperMode(1);
    setMode('record');
    setLoopLength(0);
    if (timerRef.current) clearInterval(timerRef.current);
    timerRef.current = setInterval(() => {
      setLoopLength((prev) => prev + 0.1);
    }, 100);
  }, [isTauri]);

  const stopRecording = useCallback(async () => {
    if (isTauri) await triggerLooperMode(3);
    setMode('play');
    if (timerRef.current) clearInterval(timerRef.current);
  }, [isTauri]);

  const stopPlayback = useCallback(async () => {
    if (isTauri) await triggerLooperMode(4);
    setMode('stop');
    if (timerRef.current) clearInterval(timerRef.current);
  }, [isTauri]);

  const toggleOverdub = useCallback(async () => {
    if (!isTauri) {
      setMode((m) => m === 'overdub' ? 'play' : 'overdub');
      return;
    }
    if (mode === 'overdub') await triggerLooperMode(3);
    else await triggerLooperMode(2);
    await refresh();
  }, [isTauri, mode, refresh]);

  const clearLoop = useCallback(async () => {
    if (isTauri) {
      await looperClear();
      await refresh();
    }
    setMode('idle');
    setLoopLength(0);
    if (timerRef.current) clearInterval(timerRef.current);
  }, [isTauri, refresh]);

  const formatTime = (seconds: number) => {
    const m = Math.floor(seconds / 60);
    const s = Math.floor(seconds % 60);
    const ms = Math.floor((seconds % 1) * 100);
    return `${m}:${s.toString().padStart(2, '0')}.${ms.toString().padStart(2, '0')}`;
  };

  const isRecording = mode === 'record';
  const isPlaying = mode === 'play' || mode === 'overdub';
  const isOverdubbing = mode === 'overdub';

  return (
    <div className="border border-[var(--border)] rounded-xl p-5 bg-[var(--bg-surface)]">
      <h3 className="font-display text-sm font-bold text-[var(--text-h)] tracking-wider mb-4">LOOPER</h3>

      <div className="flex items-center justify-center mb-4">
        <div className="font-mono-data text-4xl font-bold text-[var(--text-h)] tracking-wider">
          {formatTime(loopLength)}
        </div>
      </div>

      <div className="h-2 bg-[var(--bg-elevated)] rounded-full mb-4 overflow-hidden">
        <div
          className={`h-full rounded-full transition-all duration-100 ${
            isRecording ? 'bg-[var(--danger)]' : isOverdubbing ? 'bg-[var(--warning)]' : 'bg-[var(--accent)]'
          }`}
          style={{ width: isPlaying || isRecording ? '100%' : '0%' }}
        />
      </div>

      <div className="flex items-center justify-center gap-2 mb-4">
        <span className={`w-2 h-2 rounded-full ${isRecording ? 'bg-[var(--danger)] animate-pulse' : isPlaying ? 'bg-[var(--success)] animate-pulse' : 'bg-[var(--text-muted)]'}`} />
        <span className="text-xs font-medium text-[var(--text-muted)]">
          {isRecording ? 'RECORDING' : isOverdubbing ? 'OVERDUBBING' : isPlaying ? 'PLAYING' : 'EMPTY'}
        </span>
      </div>

      <div className="grid grid-cols-2 gap-2">
        {isRecording ? (
          <button
            onClick={stopRecording}
            className="col-span-2 py-2.5 rounded-lg bg-[var(--danger)]/20 text-[var(--danger)] text-sm font-bold border border-[var(--danger)]/30 hover:bg-[var(--danger)]/30 transition-colors cursor-pointer"
          >
            STOP RECORDING
          </button>
        ) : (
          <>
            <button
              onClick={startRecording}
              disabled={isPlaying}
              className="py-2.5 rounded-lg bg-[var(--danger)]/20 text-[var(--danger)] text-sm font-bold border border-[var(--danger)]/30 hover:bg-[var(--danger)]/30 transition-colors disabled:opacity-30 cursor-pointer"
            >
              RECORD
            </button>
            <button
              onClick={toggleOverdub}
              disabled={!isPlaying}
              className={`py-2.5 rounded-lg text-sm font-bold transition-colors cursor-pointer disabled:opacity-30 ${
                isOverdubbing
                  ? 'bg-[var(--warning)]/20 text-[var(--warning)] border border-[var(--warning)]/30'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-muted)] border border-[var(--border)] hover:text-[var(--text)]'
              }`}
            >
              {isOverdubbing ? 'OVERDUB ON' : 'OVERDUB'}
            </button>
            <button
              onClick={stopPlayback}
              disabled={!isPlaying && !isRecording}
              className="py-2.5 rounded-lg bg-[var(--bg-elevated)] text-[var(--text-muted)] text-sm font-bold border border-[var(--border)] hover:text-[var(--text)] transition-colors disabled:opacity-30 cursor-pointer"
            >
              STOP
            </button>
            <button
              onClick={clearLoop}
              className="py-2.5 rounded-lg bg-[var(--bg-elevated)] text-[var(--text-muted)] text-sm font-bold border border-[var(--border)] hover:text-[var(--danger)] transition-colors cursor-pointer"
            >
              CLEAR
            </button>
          </>
        )}
      </div>
    </div>
  );
}

export function Tools() {
  const fetchStatus = useEngineStore((s) => s.fetchStatus);

  useEffect(() => {
    fetchStatus();
  }, [fetchStatus]);

  return (
    <div className="flex flex-col gap-5 max-w-[1000px]">
      <div>
        <h2 className="font-display text-xl font-bold text-[var(--text-h)] tracking-wider">TOOLS</h2>
        <p className="text-[13px] text-[var(--text-muted)] mt-1">
          Tuner, metronome, and looper for practice and performance.
        </p>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
        <TunerPanel />
        <MetronomePanel />
        <LooperPanel />
      </div>
    </div>
  );
}
