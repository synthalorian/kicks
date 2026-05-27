export interface Theme {
  name: string;
  id: string;
  colors: {
    accent: string;
    accentBg: string;
    accentBorder: string;
    bg: string;
    bgSurface: string;
    bgElevated: string;
    text: string;
    textH: string;
    textMuted: string;
    border: string;
    surfaceHover: string;
    danger: string;
    dangerBg: string;
    success: string;
    successBg: string;
    warning: string;
    warningBg: string;
    /** Glow color for neon effects */
    glow: string;
    /** Secondary accent (e.g. cyan or pink) */
    accent2: string;
  };
}

export const THEMES: Theme[] = [
  {
    name: 'Default',
    id: 'default',
    colors: {
      accent: '#e87933',
      accentBg: '#2d1b0e',
      accentBorder: '#e87933',
      bg: '#0f0f14',
      bgSurface: '#1a1a24',
      bgElevated: '#24243a',
      text: '#c8c8d0',
      textH: '#f0f0f5',
      textMuted: '#8888a0',
      border: '#2a2a3e',
      surfaceHover: '#2e2e44',
      danger: '#ef4444',
      dangerBg: '#450a0a',
      success: '#22c55e',
      successBg: '#052e16',
      warning: '#eab308',
      warningBg: '#422006',
      glow: '#e87933',
      accent2: '#38bdf8',
    },
  },
  {
    name: "Synthwave '84",
    id: 'synthwave84',
    colors: {
      accent: '#8f00ff',
      accentBg: '#1a0033',
      accentBorder: '#8f00ff',
      bg: '#0d0221',
      bgSurface: '#240037',
      bgElevated: '#36004f',
      text: '#ffffff',
      textH: '#ffff66',
      textMuted: '#b088cc',
      border: '#4a007a',
      surfaceHover: '#5a0088',
      danger: '#ff007f',
      dangerBg: '#33001a',
      success: '#00ffff',
      successBg: '#003333',
      warning: '#ffff00',
      warningBg: '#333300',
      glow: '#8f00ff',
      accent2: '#ff00ff',
    },
  },
  {
    name: 'Neon Sunset',
    id: 'neon-sunset',
    colors: {
      accent: '#ff007f',
      accentBg: '#33001a',
      accentBorder: '#ff007f',
      bg: '#14051a',
      bgSurface: '#2a0a2a',
      bgElevated: '#3d1040',
      text: '#ffe6f2',
      textH: '#ffcc00',
      textMuted: '#cc88aa',
      border: '#5a1a40',
      surfaceHover: '#6e2250',
      danger: '#ff3333',
      dangerBg: '#330000',
      success: '#00ffcc',
      successBg: '#003322',
      warning: '#ffaa00',
      warningBg: '#331a00',
      glow: '#ff007f',
      accent2: '#00ffff',
    },
  },
  {
    name: 'Cyberpunk',
    id: 'cyberpunk',
    colors: {
      accent: '#00ffff',
      accentBg: '#002233',
      accentBorder: '#00ffff',
      bg: '#0a0a12',
      bgSurface: '#12121e',
      bgElevated: '#1e1e30',
      text: '#e0e0ff',
      textH: '#ffffff',
      textMuted: '#7070a0',
      border: '#2a2a40',
      surfaceHover: '#333355',
      danger: '#ff0040',
      dangerBg: '#330010',
      success: '#00ff80',
      successBg: '#003311',
      warning: '#ffea00',
      warningBg: '#332200',
      glow: '#00ffff',
      accent2: '#ff00ff',
    },
  },
  {
    name: 'Deep Violet',
    id: 'deep-violet',
    colors: {
      accent: '#b76cf9',
      accentBg: '#2a0a40',
      accentBorder: '#b76cf9',
      bg: '#0f0518',
      bgSurface: '#1a0f2e',
      bgElevated: '#281a42',
      text: '#e8d5ff',
      textH: '#f5e6ff',
      textMuted: '#8877aa',
      border: '#3a2855',
      surfaceHover: '#453060',
      danger: '#ff4080',
      dangerBg: '#2a0015',
      success: '#66ffcc',
      successBg: '#0a2a20',
      warning: '#ffcc44',
      warningBg: '#2a2000',
      glow: '#b76cf9',
      accent2: '#44ffee',
    },
  },
];

const STORAGE_KEY = 'kicks-theme';

export function getSavedThemeId(): string {
  try {
    return localStorage.getItem(STORAGE_KEY) || 'default';
  } catch {
    return 'default';
  }
}

export function saveThemeId(id: string): void {
  try {
    localStorage.setItem(STORAGE_KEY, id);
  } catch {
    // ignore
  }
}

export function applyTheme(theme: Theme): void {
  const root = document.documentElement;
  const c = theme.colors;
  root.style.setProperty('--accent', c.accent);
  root.style.setProperty('--accent-bg', c.accentBg);
  root.style.setProperty('--accent-border', c.accentBorder);
  root.style.setProperty('--bg', c.bg);
  root.style.setProperty('--bg-surface', c.bgSurface);
  root.style.setProperty('--bg-elevated', c.bgElevated);
  root.style.setProperty('--text', c.text);
  root.style.setProperty('--text-h', c.textH);
  root.style.setProperty('--text-muted', c.textMuted);
  root.style.setProperty('--border', c.border);
  root.style.setProperty('--surface-hover', c.surfaceHover);
  root.style.setProperty('--danger', c.danger);
  root.style.setProperty('--danger-bg', c.dangerBg);
  root.style.setProperty('--success', c.success);
  root.style.setProperty('--success-bg', c.successBg);
  root.style.setProperty('--warning', c.warning);
  root.style.setProperty('--warning-bg', c.warningBg);
  root.style.setProperty('--glow', c.glow);
  root.style.setProperty('--accent2', c.accent2);
}

export function getThemeById(id: string): Theme {
  return THEMES.find((t) => t.id === id) ?? THEMES[0];
}
