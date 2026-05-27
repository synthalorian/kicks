import { useEffect } from 'react';
import { PedalBoard } from '../components/SignalChain/PedalBoard';
import { AudioFlow } from '../components/SignalChain/AudioFlow';
import { Visualizer } from '../components/SignalChain/Visualizer';
import { useEngineStore } from '../stores/engineStore';

export function SignalChain() {
  const { status, start, stop, fetchChain } = useEngineStore();

  useEffect(() => {
    fetchChain();
  }, [fetchChain]);

  return (
    <div className="flex flex-col gap-5 max-w-[1400px]">
      {/* Header row */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="font-display text-xl font-bold text-[var(--text-h)] tracking-wider">SIGNAL CHAIN</h2>
          <p className="text-[13px] text-[var(--text-muted)] mt-1">
            Drag pedals to reorder. Toggle power. Twist knobs.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <div className={`flex items-center gap-2 px-3 py-1.5 rounded-md border text-[11px] font-mono-data tracking-wide ${
            status.running
              ? 'border-[var(--success)]/30 text-[var(--success)] bg-[var(--success-bg)]'
              : 'border-[var(--danger)]/30 text-[var(--danger)] bg-[var(--danger-bg)]'
          }`}>
            <span className={`w-1.5 h-1.5 rounded-full ${status.running ? 'bg-[var(--success)] animate-pulse' : 'bg-[var(--danger)]'}`} />
            {status.running ? 'ENGINE RUNNING' : 'ENGINE STOPPED'}
          </div>
          {!status.running ? (
            <button
              onClick={start}
              className="px-4 py-2 rounded-lg bg-[var(--accent)] text-[#07070a] text-sm font-bold hover:opacity-90 transition-opacity cursor-pointer neon-button"
            >
              START ENGINE
            </button>
          ) : (
            <button
              onClick={stop}
              className="px-4 py-2 rounded-lg border border-[var(--danger)]/40 text-[var(--danger)] text-sm font-bold hover:bg-[var(--danger-bg)] transition-colors cursor-pointer"
            >
              STOP ENGINE
            </button>
          )}
        </div>
      </div>

      {/* Real-time audio visualizer */}
      <div className="border border-[var(--border)] rounded-xl p-3 bg-[var(--bg-surface)]">
        <Visualizer />
      </div>

      {/* Pedalboard */}
      <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
        <div className="flex items-center justify-between mb-3">
          <span className="font-display text-[11px] text-[var(--text-muted)] tracking-[0.15em] uppercase">Pedalboard</span>
          <span className="font-mono-data text-[10px] text-[var(--text-muted)]">Drag to reorder</span>
        </div>
        <PedalBoard />
      </div>

      {/* Dynamic audio flow visualization */}
      <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
        <div className="flex items-center justify-between mb-2">
          <span className="font-display text-[11px] text-[var(--text-muted)] tracking-[0.15em] uppercase">Signal Flow</span>
        </div>
        <AudioFlow />
      </div>
    </div>
  );
}
