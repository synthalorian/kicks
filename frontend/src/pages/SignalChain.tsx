import { PedalBoard } from '../components/SignalChain/PedalBoard';
import { AudioFlow } from '../components/SignalChain/AudioFlow';
import { useEngineStore } from '../stores/engineStore';

export function SignalChain() {
  const { status, start, stop } = useEngineStore();

  return (
    <div className="flex flex-col gap-6">
      {/* Header row */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-[var(--text-h)]">Signal Chain</h2>
          <p className="text-sm text-[var(--text-muted)] mt-1">
            Design and arrange your effect chain. Drag to reorder, click to tweak.
          </p>
        </div>
        <div className="flex items-center gap-3">
          <span
            className={`text-xs px-2 py-1 rounded-full ${
              status.running
                ? 'bg-green-900/50 text-green-400 border border-green-700'
                : 'bg-red-900/50 text-red-400 border border-red-700'
            }`}
          >
            {status.running ? 'Engine Running' : 'Engine Stopped'}
          </span>
          {!status.running ? (
            <button
              onClick={start}
              className="px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity cursor-pointer"
            >
              Start Engine
            </button>
          ) : (
            <button
              onClick={stop}
              className="px-4 py-2 rounded-lg border border-red-700 text-red-400 text-sm font-medium hover:bg-red-900/30 transition-colors cursor-pointer"
            >
              Stop Engine
            </button>
          )}
        </div>
      </div>

      {/* Pedalboard */}
      <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
        <PedalBoard />
      </div>

      {/* Dynamic audio flow visualization */}
      <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
        <AudioFlow />
      </div>
    </div>
  );
}
