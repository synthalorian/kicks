interface StatusBarProps {
  version: string;
  engineStatus: 'disconnected' | 'connecting' | 'connected';
}

export function StatusBar({ version, engineStatus }: StatusBarProps) {
  return (
    <footer className="flex items-center justify-between px-4 py-1 border-t border-[var(--border)] bg-[var(--bg-surface)] text-xs text-[var(--text-muted)] shrink-0">
      <span>
        JACK:{' '}
        <span
          className={
            engineStatus === 'connected' ? 'text-green-400' : 'text-red-400'
          }
        >
          {engineStatus === 'connected' ? 'Connected' : 'Not connected'}
        </span>
      </span>
      <span>Kicks v{version}</span>
    </footer>
  );
}
