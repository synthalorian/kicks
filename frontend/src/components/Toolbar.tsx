interface ToolbarProps {
  engineStatus: 'disconnected' | 'connecting' | 'connected';
}

export function Toolbar({ engineStatus }: ToolbarProps) {
  const dotColor =
    engineStatus === 'connected'
      ? 'bg-green-500'
      : engineStatus === 'connecting'
        ? 'bg-yellow-500'
        : 'bg-red-500';

  const label =
    engineStatus === 'connected'
      ? 'Guitarix Connected'
      : engineStatus === 'connecting'
        ? 'Connecting...'
        : 'Disconnected';

  return (
    <header className="flex items-center justify-between px-4 py-2 border-b border-[var(--border)] bg-[var(--bg-surface)] shrink-0">
      <div className="flex items-center gap-3">
        <h1 className="text-lg font-bold text-[var(--accent)] tracking-tight">
          KICKS
        </h1>
        <span className="text-xs text-[var(--text-muted)]">Guitar Workstation</span>
      </div>
      <div className="flex items-center gap-2">
        <span className={`w-2 h-2 rounded-full ${dotColor}`} />
        <span className="text-xs text-[var(--text-muted)]">{label}</span>
      </div>
    </header>
  );
}
