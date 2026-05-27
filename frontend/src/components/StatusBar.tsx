interface StatusBarProps {
  version: string;
  engineStatus: 'disconnected' | 'connecting' | 'connected';
}

export function StatusBar({ version, engineStatus }: StatusBarProps) {
  return (
    <footer className="flex items-center justify-between px-5 py-1.5 border-t border-[var(--border)] bg-[var(--bg-surface)]/90 backdrop-blur text-[11px] text-[var(--text-muted)] shrink-0 font-mono-data tracking-wide">
      <div className="flex items-center gap-4">
        <span className="flex items-center gap-1.5">
          <span className={`w-1.5 h-1.5 rounded-full ${engineStatus === 'connected' ? 'bg-[var(--success)]' : 'bg-[var(--danger)]'}`} />
          JACK {engineStatus === 'connected' ? 'ACTIVE' : 'INACTIVE'}
        </span>
        <span className="text-[var(--border)]">|</span>
        <span>CPU — 0.0%</span>
        <span className="text-[var(--border)]">|</span>
        <span>48.0 kHz</span>
        <span className="text-[var(--border)]">|</span>
        <span>256 smp</span>
      </div>
      <span className="text-[var(--text-muted)]/60">v{version}</span>
    </footer>
  );
}
