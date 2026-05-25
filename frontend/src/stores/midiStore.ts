import { create } from 'zustand';
import type { MidiConfigPayload, MidiDeviceInfo, MidiEventReport } from '../types/tauri';
import * as api from '../lib/tauri';

interface MidiState {
  config: MidiConfigPayload | null;
  devices: MidiDeviceInfo[];
  learnEvents: MidiEventReport[];
  connected: boolean;
  loading: boolean;

  fetchConfig: () => Promise<void>;
  saveConfig: (config: MidiConfigPayload) => Promise<void>;
  fetchDevices: () => Promise<void>;
  connectDevice: (name: string) => Promise<void>;
  disconnectDevice: () => Promise<void>;
  pollEvents: () => Promise<MidiEventReport[]>;
  setLearn: (active: boolean) => Promise<void>;
}

export const useMidiStore = create<MidiState>((set, get) => ({
  config: null,
  devices: [],
  learnEvents: [],
  connected: false,
  loading: false,

  fetchConfig: async () => {
    try {
      const config = await api.getMidiConfig();
      set({ config });
    } catch (err) {
      console.error('Failed to fetch MIDI config:', err);
    }
  },

  saveConfig: async (config) => {
    set({ loading: true });
    try {
      await api.saveMidiConfig(config);
      set({ config });
    } finally {
      set({ loading: false });
    }
  },

  fetchDevices: async () => {
    try {
      const devices = await api.listMidiDevices();
      set({ devices });
    } catch (err) {
      console.error('Failed to list MIDI devices:', err);
    }
  },

  connectDevice: async (name) => {
    set({ loading: true, connected: false });
    try {
      await api.connectMidiDevice(name);
      // Update config active_device
      const config = get().config;
      if (config) {
        set({ config: { ...config, active_device: name } });
      }
      set({ connected: true });
    } catch (err) {
      console.error('Failed to connect MIDI device:', err);
      throw err;
    } finally {
      set({ loading: false });
    }
  },

  disconnectDevice: async () => {
    set({ loading: true });
    try {
      await api.disconnectMidiDevice();
      const config = get().config;
      if (config) {
        set({ config: { ...config, active_device: null } });
      }
      set({ connected: false });
    } catch (err) {
      console.error('Failed to disconnect MIDI device:', err);
    } finally {
      set({ loading: false });
    }
  },

  pollEvents: async () => {
    try {
      const events = await api.pollMidiEvents();
      // If in learn mode, accumulate events for display
      const config = get().config;
      if (config?.learn_mode && events.length > 0) {
        const learnEvents = events.map((e) => ({
          ...e,
          // Only show one event per CC for learn UI
        }));
        // Keep last 5 events
        set({ learnEvents: [...get().learnEvents, ...learnEvents].slice(-5) });

        // If we got a learn event, update last_cc in local config
        const lastEvent = events[events.length - 1];
        if (lastEvent && !lastEvent.mapped_parameter) {
          set({ config: { ...config, last_cc: lastEvent.cc } });
        }
      }
      return events;
    } catch (err) {
      console.error('Failed to poll MIDI events:', err);
      return [];
    }
  },

  setLearn: async (active) => {
    try {
      await api.setMidiLearn(active);
      const config = get().config;
      if (config) {
        set({
          config: { ...config, learn_mode: active, last_cc: active ? config.last_cc : null },
          learnEvents: active ? get().learnEvents : [],
        });
      }
    } catch (err) {
      console.error('Failed to set MIDI learn:', err);
    }
  },
}));
