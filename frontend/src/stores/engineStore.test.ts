import { describe, it, expect } from 'vitest';
import { useEngineStore } from './engineStore';

describe('engineStore', () => {
  it('has correct initial state', () => {
    const state = useEngineStore.getState();
    expect(state.status.running).toBe(false);
    expect(state.status.sample_rate).toBe(0);
    expect(state.status.buffer_size).toBe(0);
    expect(state.chain).toBeNull();
    expect(state.loading).toBe(false);
    expect(state.levels).toEqual([]);
    expect(state.levelsError).toBeNull();
  });

  it('exposes all expected actions', () => {
    const state = useEngineStore.getState();
    expect(typeof state.fetchStatus).toBe('function');
    expect(typeof state.start).toBe('function');
    expect(typeof state.stop).toBe('function');
    expect(typeof state.fetchChain).toBe('function');
    expect(typeof state.buildDefault).toBe('function');
    expect(typeof state.updateSlot).toBe('function');
    expect(typeof state.toggleSlot).toBe('function');
    expect(typeof state.moveSlot).toBe('function');
    expect(typeof state.setParameter).toBe('function');
    expect(typeof state.undo).toBe('function');
    expect(typeof state.redo).toBe('function');
    expect(typeof state.pollLevels).toBe('function');
  });
});
