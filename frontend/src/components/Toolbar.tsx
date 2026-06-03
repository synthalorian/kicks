import { useEngineStore } from '../stores/engineStore';

interface ToolbarProps {
  engineStatus: 'disconnected' | 'connecting' | 'connected';
  modeLabel?: string;
}

export function Toolbar({ engineStatus, modeLabel }: ToolbarProps) {
  const isConnected = engineStatus === 'connected';
  const isConnecting = engineStatus === 'connecting';
  const status = useEngineStore((s) => s.status);
  const loading = useEngineStore((s) => s.loading);
  const start = useEngineStore((s) => s.start);
  const stop = useEngineStore((s) => s.stop);

  const handleToggle = () => {
    if (isConnected) stop();
    else start();
  };

  return (
    <header data-testid="toolbar" className="flex items-center justify-between px-5 py-2.5 border-b border-[var(--border)] bg-[var(--bg-surface)]/90 backdrop-blur shrink-0">
      <div className="flex items-center gap-4">
        <h1 className="font-display text-xl font-black text-[var(--accent)] tracking-[0.15em] neon-text">
          KICKS
        </h1>
        <span className="text-[11px] text-[var(--text-muted)] tracking-wider uppercase hidden sm:inline">
          Guitar Workstation
        </span>
        {modeLabel && (
          <span className="font-mono-data text-[10px] px-2 py-0.5 rounded border border-[var(--border)] text-[var(--text-muted)] tracking-wider">
            {modeLabel}
          </span>
        )}
      </div>
      <div className="flex items-center gap-3">
        {/* Engine mode badge */}
        {status.mode !== 'none' && status.mode !== 'browser' && (
          <span className="font-mono-data text-[10px] px-2 py-0.5 rounded border border-[var(--border)] text-[var(--text-muted)] tracking-wider uppercase">
            {status.mode}
          </span>
        )}
        {/* Start/Stop button */}
        <button
          onClick={handleToggle}
          disabled={loading}
          className={`px-3 py-1.5 rounded-md text-[11px] font-medium tracking-wide transition-all cursor-pointer border ${
            isConnected
              ? 'border-[var(--danger)] text-[var(--danger)] hover:bg-[var(--danger)]/10'
              : 'border-[var(--success)] text-[var(--success)] hover:bg-[var(--success)]/10'
          } disabled:opacity-40`}
        >
          {loading ? '...' : isConnected ? 'STOP' : 'START'}
        </button>
        {/* Status indicator */}
        <div className="flex items-center gap-2 px-2.5 py-1 rounded-md border border-[var(--border)] bg-[var(--bg-elevated)]/50">
          <span
            className={`w-1.5 h-1.5 rounded-full ${
              isConnected
                ? 'bg-[var(--success)] animate-pulse'
                : isConnecting
                  ? 'bg-[var(--warning)]'
                  : 'bg-[var(--danger)]'
            }`}
          />
          <span className="text-[11px] text-[var(--text-muted)] font-mono-data tracking-wide">
            {isConnected ? 'RUNNING' : isConnecting ? 'CONNECTING' : 'STOPPED'}
          </span>
        </div>
      </div>
    </header>
  );
}
