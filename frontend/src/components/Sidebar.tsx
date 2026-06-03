export type Page =
  | 'signal-chain'
  | 'presets'
  | 'ir-browser'
  | 'nam-browser'
  | 'midi'
  | 'live'
  | 'tools'
  | 'ai-assistant'
  | 'settings';

interface SidebarProps {
  activePage: Page;
  onNavigate: (page: Page) => void;
  onShowWizard?: () => void;
}

const navItems: { id: Page; label: string; icon: string; kbd: string }[] = [
  { id: 'signal-chain', label: 'Signal Chain', icon: '⛓', kbd: '1' },
  { id: 'presets', label: 'Presets', icon: '💾', kbd: '2' },
  { id: 'ir-browser', label: 'IR Browser', icon: '🔊', kbd: '3' },
  { id: 'nam-browser', label: 'NAM Models', icon: '🧠', kbd: '4' },
  { id: 'midi', label: 'MIDI', icon: '🎹', kbd: '5' },
  { id: 'live', label: 'Live', icon: '🎸', kbd: '6' },
  { id: 'tools', label: 'Tools', icon: '🛠', kbd: '7' },
  { id: 'ai-assistant', label: 'AI Assistant', icon: '✨', kbd: '8' },
  { id: 'settings', label: 'Settings', icon: '⚙', kbd: '9' },
];

export function Sidebar({ activePage, onNavigate, onShowWizard }: SidebarProps) {
  return (
    <nav className="w-52 border-r border-[var(--border)] bg-[var(--bg-surface)]/80 backdrop-blur flex flex-col gap-0.5 p-2 shrink-0">
      <div className="px-3 py-2 mb-2">
        <div className="font-display text-[10px] tracking-[0.2em] text-[var(--text-muted)] uppercase">
          Navigation
        </div>
      </div>
      {navItems.map((item) => {
        const isActive = activePage === item.id;
        return (
          <button
            key={item.id}
            onClick={() => onNavigate(item.id)}
            className={`group text-left px-3 py-2 rounded-lg text-sm transition-all cursor-pointer relative flex items-center gap-2.5 ${
              isActive
                ? 'bg-[var(--accent-bg)] text-[var(--accent)] font-semibold'
                : 'text-[var(--text)] hover:bg-[var(--surface-hover)]'
            }`}
          >
            {isActive && (
              <span
                className="absolute left-0 top-1/2 -translate-y-1/2 w-0.5 h-4 rounded-full"
                style={{ background: 'var(--accent)' }}
              />
            )}
            <span className="text-base opacity-80 group-hover:opacity-100 transition-opacity">
              {item.icon}
            </span>
            <span className="flex-1">{item.label}</span>
            <span
              className={`text-[10px] px-1 py-0.5 rounded font-mono-data ${
                isActive
                  ? 'bg-[var(--accent)]/20 text-[var(--accent)]'
                  : 'bg-[var(--bg-elevated)] text-[var(--text-muted)]'
              }`}
            >
              {item.kbd}
            </span>
          </button>
        );
      })}

      <div className="mt-auto">
        {onShowWizard && (
          <button
            onClick={onShowWizard}
            className="w-full text-left px-3 py-2 rounded-lg text-sm text-[var(--text-muted)] hover:text-[var(--accent)] hover:bg-[var(--accent-bg)] transition-all cursor-pointer flex items-center gap-2.5 mb-2"
          >
            <span className="text-base opacity-80">❓</span>
            <span>Setup Wizard</span>
          </button>
        )}
        <div className="px-3 py-3">
          <div className="text-[9px] text-[var(--text-muted)] font-mono-data leading-relaxed">
            <div>Space — toggle engine</div>
            <div>Ctrl+Z — undo</div>
            <div>Ctrl+Shift+Z — redo</div>
          </div>
        </div>
      </div>
    </nav>
  );
}
