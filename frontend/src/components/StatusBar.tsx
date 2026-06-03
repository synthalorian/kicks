import { useEngineStore } from '../stores/engineStore';

interface StatusBarProps {
  version: string;
  engineStatus: 'disconnected' | 'connecting' | 'connected';
}

export function StatusBar({ version, engineStatus }: StatusBarProps) {
  const status = useEngineStore((s) => s.status);
  const cpuLoad = useEngineStore((s) => s.cpuLoad);

  return (
    <footer
      data-testid="status-bar"
      className="flex items-center justify-between px-5 py-1.5 border-t border-[var(--border)] bg-[var(--bg-surface)]/90 backdrop-blur text-[11px] text-[var(--text-muted)] shrink-0"
    >
      <div className="flex items-center gap-4"
      >
        <span className="font-mono-data tracking-wider"
        >
          v{version}
        </span>
        <span className="hidden sm:inline text-[var(--border)]"
        >|</span>
        <div className="hidden sm:flex items-center gap-3"
        >
          <span className="flex items-center gap-1.5"
          >
            <span
              className={`w-1.5 h-1.5 rounded-full ${engineStatus === 'connected' ? 'bg-[var(--success)]' : 'bg-[var(--danger)]'}`}
            />
            ENGINE {engineStatus === 'connected' ? 'ACTIVE' : 'INACTIVE'}
          </span>
          {status.running && (
            <>
              <span className="text-[var(--border)]"
              >|</span>
              <span className="font-mono-data"
              >
                {status.sample_rate / 1000}kHz / {status.buffer_size}
              </span>
              <span className="text-[var(--border)]"
              >|</span>
              <span className="font-mono-data uppercase"
              >
                {status.backend}
              </span>
              {status.mode !== 'none' && status.mode !== 'browser' && (
                <>
                  <span className="text-[var(--border)]"
                  >|</span>
                  <span className="font-mono-data uppercase text-[var(--accent)]"
                  >
                    {status.mode}
                  </span>
                </>
              )}
            </>
          )}
        </div>
      </div>
      <div className="flex items-center gap-3"
      >
        {cpuLoad > 0 && (
          <span className="font-mono-data"
          >
            CPU {cpuLoad.toFixed(1)}%
          </span>
        )}
        <span className="hidden sm:inline"
        >
          Space = Start/Stop
        </span>
      </div>
    </footer>
  );
}
