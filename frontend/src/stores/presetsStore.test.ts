import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as api from '../lib/tauri';
import { usePresetsStore } from './presetsStore';

vi.mock('../lib/tauri', () => ({
  listPresets: vi.fn(),
  savePreset: vi.fn(),
  loadPreset: vi.fn(),
  deletePreset: vi.fn(),
  renamePreset: vi.fn(),
}));

const mockedApi = vi.mocked(api);

describe('usePresetsStore', () => {
  beforeEach(() => {
    usePresetsStore.setState({
      banks: [],
      loading: false,
    });
    vi.clearAllMocks();
  });

  it('initializes with empty banks', () => {
    const state = usePresetsStore.getState();
    expect(state.banks).toEqual([]);
    expect(state.loading).toBe(false);
  });

  it('fetchPresets loads banks', async () => {
    const banks = [
      {
        name: 'Factory',
        presets: [
          { name: 'Clean', description: '', tags: [], created: '', modified: '' },
        ],
      },
    ];
    mockedApi.listPresets.mockResolvedValueOnce(banks);

    await usePresetsStore.getState().fetchPresets();

    expect(mockedApi.listPresets).toHaveBeenCalledOnce();
    expect(usePresetsStore.getState().banks).toEqual(banks);
    expect(usePresetsStore.getState().loading).toBe(false);
  });

  it('savePreset calls API and refreshes banks', async () => {
    mockedApi.savePreset.mockResolvedValueOnce(undefined);
    mockedApi.listPresets.mockResolvedValueOnce([]);

    await usePresetsStore.getState().savePreset('User', 'Lead', 'My lead tone', ['lead']);

    expect(mockedApi.savePreset).toHaveBeenCalledWith('User', 'Lead', 'My lead tone', ['lead']);
    expect(mockedApi.listPresets).toHaveBeenCalledOnce();
    expect(usePresetsStore.getState().loading).toBe(false);
  });

  it('loadPreset calls API and resets loading', async () => {
    mockedApi.loadPreset.mockResolvedValueOnce(undefined);

    await usePresetsStore.getState().loadPreset('Factory', 'Clean');

    expect(mockedApi.loadPreset).toHaveBeenCalledWith('Factory', 'Clean');
    expect(usePresetsStore.getState().loading).toBe(false);
  });

  it('deletePreset calls API and refreshes banks', async () => {
    mockedApi.deletePreset.mockResolvedValueOnce(undefined);
    mockedApi.listPresets.mockResolvedValueOnce([]);

    await usePresetsStore.getState().deletePreset('User', 'Old');

    expect(mockedApi.deletePreset).toHaveBeenCalledWith('User', 'Old');
    expect(mockedApi.listPresets).toHaveBeenCalledOnce();
    expect(usePresetsStore.getState().loading).toBe(false);
  });

  it('renamePreset calls API and refreshes banks', async () => {
    mockedApi.renamePreset.mockResolvedValueOnce(undefined);
    mockedApi.listPresets.mockResolvedValueOnce([]);

    await usePresetsStore.getState().renamePreset('User', 'Old', 'New');

    expect(mockedApi.renamePreset).toHaveBeenCalledWith('User', 'Old', 'New');
    expect(mockedApi.listPresets).toHaveBeenCalledOnce();
    expect(usePresetsStore.getState().loading).toBe(false);
  });

  it('sets loading while fetching', async () => {
    let resolveList: (value: typeof api.listPresets extends () => Promise<infer T> ? T : never) => void;
    mockedApi.listPresets.mockImplementationOnce(
      () => new Promise((r) => { resolveList = r as unknown as typeof resolveList; })
    );

    const promise = usePresetsStore.getState().fetchPresets();
    expect(usePresetsStore.getState().loading).toBe(true);

    resolveList!([{ name: 'Bank', presets: [] }]);
    await promise;

    expect(usePresetsStore.getState().loading).toBe(false);
  });
});
