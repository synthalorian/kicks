export type Page =
  | 'signal-chain'
  | 'presets'
  | 'ir-browser'
  | 'midi'
  | 'live'
  | 'ai-assistant'
  | 'settings';

interface SidebarProps {
  activePage: Page;
  onNavigate: (page: Page) => void;
}

const navItems: { id: Page; label: string }[] = [
  { id: 'signal-chain', label: 'Signal Chain' },
  { id: 'presets', label: 'Presets' },
  { id: 'ir-browser', label: 'IR Browser' },
  { id: 'midi', label: 'MIDI' },
  { id: 'live', label: 'Live' },
  { id: 'ai-assistant', label: 'AI Assistant' },
  { id: 'settings', label: 'Settings' },
];

export function Sidebar({ activePage, onNavigate }: SidebarProps) {
  return (
    <nav className="w-48 border-r border-[var(--border)] bg-[var(--bg-surface)] p-2 flex flex-col gap-1">
      {navItems.map((item) => (
        <button
          key={item.id}
          onClick={() => onNavigate(item.id)}
          className={`text-left px-3 py-2 rounded text-sm transition-colors cursor-pointer ${
            activePage === item.id
              ? 'bg-[var(--accent)] text-white font-medium'
              : 'text-[var(--text)] hover:bg-[var(--surface-hover)]'
          }`}
        >
          {item.label}
        </button>
      ))}
    </nav>
  );
}
