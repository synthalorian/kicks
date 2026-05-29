import { useEffect } from 'react';
import { useEngineStore } from '../stores/engineStore';

interface StatusBarProps {
  version: string;
  engineStatus: 'disconnected' | 'connecting' | 'connected';
}

export function StatusBar({ version, engineStatus }: StatusBarProps) {
  const cpuLoad = useEngineStore((s) => s.cpuLoad);
  const status = useEngineStore((s) => s.status);
  const pollCpu = useEngineStore((s) => s.pollCpu);

  useEffect(() => {
    if (!status.running) return;
    const id = setInterval(() => {
      pollCpu();
    }, 500);
    return () => clearInterval(id);
  }, [status.running, pollCpu]);

  return (
    <footer className="flex items-center justify-between px-5 py-1.5 border-t border-[var(--border)] bg-[var(--bg-surface)]/90 backdrop-blur text-[11px] text-[var(--text-muted)] shrink-0 font-mono-data tracking-wide">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5">
          <span className={`w-1.5 h-1.5 rounded-full ${engineStatus === 'connected' ? 'bg-[var(--success)]' : 'bg-[var(--danger)]'}`} />
          ENGINE {engineStatus === 'connected' ? 'ACTIVE' : 'INACTIVE'}
        </span>
        <span className="text-[var(--border)]">|</span>
        <span>CPU {cpuLoad > 0 ? `${cpuLoad.toFixed(1)}%` : '— —%'}</span>
        <span className="text-[var(--border)]">|</span>
        <span>{status.running ? (status.sample_rate / 1000).toFixed(1) : '— —'} kHz</span>
        <span className="text-[var(--border)]">|</span>
        <span>{status.running ? status.buffer_size : '— —'} smp</span>
      </div>
      <span className="text-[var(--text-muted)]/60">v{version}</span>
    </footer>
  );
}
