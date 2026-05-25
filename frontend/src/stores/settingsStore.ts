import { create } from 'zustand';
import type { SettingsPayload } from '../types/tauri';
import * as api from '../lib/tauri';

interface SettingsState {
  settings: SettingsPayload | null;
  loading: boolean;
  dirty: boolean;

  /** Fetch current settings from backend */
  fetchSettings: () => Promise<void>;
  /** Save settings to backend */
  saveSettings: (settings: SettingsPayload) => Promise<void>;
  /** Update a local setting field without saving */
  updateSetting: <K extends keyof SettingsPayload>(key: K, value: SettingsPayload[K]) => void;
}

export const useSettingsStore = create<SettingsState>((set, get) => ({
  settings: null,
  loading: false,
  dirty: false,

  fetchSettings: async () => {
    set({ loading: true });
    try {
      const settings = await api.getSettings();
      set({ settings, dirty: false });
    } finally {
      set({ loading: false });
    }
  },

  saveSettings: async (settings) => {
    set({ loading: true });
    try {
      await api.saveSettings(settings);
      set({ settings, dirty: false });
    } finally {
      set({ loading: false });
    }
  },

  updateSetting: (key, value) => {
    const current = get().settings;
    if (current) {
      set({ settings: { ...current, [key]: value }, dirty: true });
    }
  },
}));
