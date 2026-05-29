import { create } from 'zustand';
import type { EngineStatus, ChainSnapshot } from '../types/tauri';
import * as api from '../lib/tauri';
import { webAudioEngine } from '../audio/audioEngine';

function isTauriAvailable(): boolean {
  return !!(window as unknown as Record<string, unknown>).__TAURI__;
}

interface EngineState {
  status: EngineStatus;
  chain: ChainSnapshot | null;
  loading: boolean;
  /** Per-plugin RMS audio levels (0..1), updated via polling. */
  levels: number[];
  /** Error from the last level poll. */
  levelsError: string | null;
  /** CPU load percentage (0.0 – 100.0+), updated via polling. */
  cpuLoad: number;
  /** Error from the last CPU load poll. */
  cpuError: string | null;
  /** Whether the app is running inside Tauri (real Rust backend) or browser (Web Audio). */
  isTauri: boolean;

  /** Fetch engine status from backend */
  fetchStatus: () => Promise<void>;
  /** Start the audio engine */
  start: () => Promise<void>;
  /** Stop the audio engine */
  stop: () => Promise<void>;
  /** Fetch the current signal chain */
  fetchChain: () => Promise<void>;
  /** Reset to the default signal chain */
  buildDefault: () => Promise<void>;
  /** Update a slot's parameters */
  updateSlot: (slotId: string, changes: { enabled?: boolean; wet_dry?: number; parameters?: Record<string, number> }) => Promise<void>;
  /** Toggle a slot on/off */
  toggleSlot: (slotId: string) => Promise<void>;
  /** Move a slot to a new position (drag-and-drop reorder) */
  moveSlot: (fromIdx: number, toIdx: number) => Promise<void>;
  /** Set a parameter on the current signal chain */
  setParameter: (id: string, value: number) => Promise<void>;
  /** Undo the last signal chain change */
  undo: () => Promise<void>;
  /** Redo a previously undone signal chain change */
  redo: () => Promise<void>;
  /** Poll the latest audio levels from the backend. */
  pollLevels: () => Promise<void>;
  /** Poll the latest CPU load from the backend. */
  pollCpu: () => Promise<void>;
}

export const useEngineStore = create<EngineState>((set, get) => ({
  status: { running: false, sample_rate: 0, buffer_size: 0 },
  chain: null,
  loading: false,
  levels: [],
  levelsError: null,
  cpuLoad: 0,
  cpuError: null,
  isTauri: isTauriAvailable(),

  fetchStatus: async () => {
    if (get().isTauri) {
      const status = await api.engineStatus();
      set({ status });
    } else {
      const state = webAudioEngine.getState();
      set({ status: { running: state.running, sample_rate: state.sampleRate, buffer_size: state.bufferSize } });
    }
  },

  start: async () => {
    set({ loading: true });
    try {
      if (get().isTauri) {
        await api.startEngine();
        await get().fetchStatus();
        await get().fetchChain();
      } else {
        await webAudioEngine.start();
        const state = webAudioEngine.getState();
        const chain = await api.getSignalChain();
        webAudioEngine.applyChain(chain);
        set({ status: { running: state.running, sample_rate: state.sampleRate, buffer_size: state.bufferSize }, chain });
      }
    } finally {
      set({ loading: false });
    }
  },

  stop: async () => {
    set({ loading: true });
    try {
      if (get().isTauri) {
        await api.stopEngine();
      } else {
        await webAudioEngine.stop();
      }
      set({ status: { running: false, sample_rate: 0, buffer_size: 0 } });
    } finally {
      set({ loading: false });
    }
  },

  fetchChain: async () => {
    const chain = await api.getSignalChain();
    if (!get().isTauri && get().status.running) {
      webAudioEngine.applyChain(chain);
    }
    set({ chain });
  },

  buildDefault: async () => {
    await api.buildDefaultChain();
    await get().fetchChain();
  },

  updateSlot: async (slotId, changes) => {
    if (get().isTauri) {
      await api.updateSlot(slotId, changes);
      await get().fetchChain();
    } else {
      const chain = get().chain;
      if (!chain) return;
      const slot = chain.slots.find((s) => s.id === slotId);
      if (!slot) return;
      if (changes.enabled !== undefined) {
        slot.enabled = changes.enabled;
        webAudioEngine.toggleSlot(slotId, changes.enabled);
      }
      if (changes.parameters) {
        Object.entries(changes.parameters).forEach(([k, v]) => {
          slot.parameters[k] = v;
          webAudioEngine.setParameter(slotId, k, v);
        });
      }
      if (changes.wet_dry !== undefined) {
        slot.wet_dry = changes.wet_dry;
      }
      set({ chain: { ...chain, slots: [...chain.slots] } });
    }
  },

  toggleSlot: async (slotId) => {
    if (get().isTauri) {
      await api.toggleSlot(slotId);
      await get().fetchChain();
    } else {
      const chain = get().chain;
      if (!chain) return;
      const slot = chain.slots.find((s) => s.id === slotId);
      if (slot) {
        slot.enabled = !slot.enabled;
        webAudioEngine.toggleSlot(slotId, slot.enabled);
        set({ chain: { ...chain, slots: [...chain.slots] } });
      }
    }
  },

  moveSlot: async (fromIdx, toIdx) => {
    if (get().isTauri) {
      await api.moveSlot(fromIdx, toIdx);
      await get().fetchChain();
    } else {
      const chain = get().chain;
      if (!chain) return;
      const slots = [...chain.slots];
      const [moved] = slots.splice(fromIdx, 1);
      if (!moved) return;
      slots.splice(toIdx, 0, moved);
      set({ chain: { ...chain, slots } });
    }
  },

  setParameter: async (id, value) => {
    if (get().isTauri) {
      await api.setParameter(id, value);
    } else {
      const chain = get().chain;
      if (!chain) return;

      // Try to parse "slotId.paramId" format
      let slotId: string | null = null;
      let paramId: string | null = null;

      const parts = id.split('.');
      if (parts.length === 2 && chain.slots.some((s) => s.id === parts[0])) {
        slotId = parts[0];
        paramId = parts[1];
      } else {
        // Search for a slot that has this parameter
        for (const slot of chain.slots) {
          if (id in slot.parameters) {
            slotId = slot.id;
            paramId = id;
            break;
          }
        }
      }

      if (!slotId || !paramId) return;

      const slot = chain.slots.find((s) => s.id === slotId);
      if (!slot || !(paramId in slot.parameters)) return;

      slot.parameters[paramId] = value;
      webAudioEngine.setParameter(slotId, paramId, value);
      set({ chain: { ...chain, slots: [...chain.slots] } });
    }
  },

  pollLevels: async () => {
    if (get().isTauri) {
      try {
        const levels = await api.getAudioLevels();
        set({ levels, levelsError: null });
      } catch (err) {
        set({ levels: [], levelsError: String(err) });
      }
    } else {
      const levels = webAudioEngine.getLevels();
      set({ levels, levelsError: null });
    }
  },

  pollCpu: async () => {
    if (get().isTauri) {
      try {
        const cpu = await api.getCpuLoad();
        set({ cpuLoad: cpu, cpuError: null });
      } catch (err) {
        set({ cpuLoad: 0, cpuError: String(err) });
      }
    }
  },

  undo: async () => {
    try {
      await api.undoSignalChain();
      await get().fetchChain();
    } catch {
      // Nothing to undo
    }
  },

  redo: async () => {
    try {
      await api.redoSignalChain();
      await get().fetchChain();
    } catch {
      // Nothing to redo
    }
  },
}));
