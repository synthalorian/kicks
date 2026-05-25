import { create } from 'zustand';
import type { BankDescriptor } from '../types/tauri';
import * as api from '../lib/tauri';

interface PresetsState {
  banks: BankDescriptor[];
  loading: boolean;

  /** Fetch all presets from backend */
  fetchPresets: () => Promise<void>;
  /** Save current signal chain as a preset */
  savePreset: (bankName: string, presetName: string, description?: string, tags?: string[]) => Promise<void>;
  /** Load a preset into the signal chain */
  loadPreset: (bankName: string, presetName: string) => Promise<void>;
  /** Delete a preset */
  deletePreset: (bankName: string, presetName: string) => Promise<void>;
  /** Rename a preset */
  renamePreset: (bankName: string, oldName: string, newName: string) => Promise<void>;
}

export const usePresetsStore = create<PresetsState>((set, get) => ({
  banks: [],
  loading: false,

  fetchPresets: async () => {
    set({ loading: true });
    try {
      const banks = await api.listPresets();
      set({ banks });
    } finally {
      set({ loading: false });
    }
  },

  savePreset: async (bankName, presetName, description, tags) => {
    set({ loading: true });
    try {
      await api.savePreset(bankName, presetName, description, tags);
      await get().fetchPresets();
    } finally {
      set({ loading: false });
    }
  },

  loadPreset: async (bankName, presetName) => {
    set({ loading: true });
    try {
      await api.loadPreset(bankName, presetName);
    } finally {
      set({ loading: false });
    }
  },

  deletePreset: async (bankName, presetName) => {
    set({ loading: true });
    try {
      await api.deletePreset(bankName, presetName);
      await get().fetchPresets();
    } finally {
      set({ loading: false });
    }
  },

  renamePreset: async (bankName, oldName, newName) => {
    set({ loading: true });
    try {
      await api.renamePreset(bankName, oldName, newName);
      await get().fetchPresets();
    } finally {
      set({ loading: false });
    }
  },
}));
