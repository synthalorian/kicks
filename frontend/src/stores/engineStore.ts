import { create } from 'zustand';
import type { EngineStatus, ChainSnapshot } from '../types/tauri';
import * as api from '../lib/tauri';

interface EngineState {
  status: EngineStatus;
  chain: ChainSnapshot | null;
  loading: boolean;
  /** Per-plugin RMS audio levels (0..1), updated via polling. */
  levels: number[];
  /** Error from the last level poll (e.g. engine not running). */
  levelsError: string | null;

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
}

export const useEngineStore = create<EngineState>((set, get) => ({
  status: { running: false, sample_rate: 0, buffer_size: 0 },
  chain: null,
  loading: false,
  levels: [],
  levelsError: null,

  fetchStatus: async () => {
    const status = await api.engineStatus();
    set({ status });
  },

  start: async () => {
    set({ loading: true });
    try {
      await api.startEngine();
      await get().fetchStatus();
      await get().fetchChain();
    } finally {
      set({ loading: false });
    }
  },

  stop: async () => {
    set({ loading: true });
    try {
      await api.stopEngine();
      set({ status: { running: false, sample_rate: 0, buffer_size: 0 } });
    } finally {
      set({ loading: false });
    }
  },

  fetchChain: async () => {
    const chain = await api.getSignalChain();
    set({ chain });
  },

  buildDefault: async () => {
    await api.buildDefaultChain();
    await get().fetchChain();
  },

  updateSlot: async (slotId, changes) => {
    await api.updateSlot(slotId, changes);
    await get().fetchChain();
  },

  toggleSlot: async (slotId) => {
    await api.toggleSlot(slotId);
    await get().fetchChain();
  },

  moveSlot: async (fromIdx, toIdx) => {
    await api.moveSlot(fromIdx, toIdx);
    await get().fetchChain();
  },

  setParameter: async (id, value) => {
    await api.setParameter(id, value);
  },

  pollLevels: async () => {
    try {
      const levels = await api.getAudioLevels();
      set({ levels, levelsError: null });
    } catch (err) {
      // Engine not running — clear levels
      set({ levels: [], levelsError: String(err) });
    }
  },

  undo: async () => {
    try {
      await api.undoSignalChain();
      await get().fetchChain();
    } catch {
      // Nothing to undo — silently ignored
    }
  },

  redo: async () => {
    try {
      await api.redoSignalChain();
      await get().fetchChain();
    } catch {
      // Nothing to redo — silently ignored
    }
  },
}));
