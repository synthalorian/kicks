import { describe, it, expect, vi, beforeEach } from 'vitest';
import * as api from '../lib/tauri';
import { useMidiStore } from './midiStore';

vi.mock('../lib/tauri', () => ({
  getMidiConfig: vi.fn(),
  saveMidiConfig: vi.fn(),
  listMidiDevices: vi.fn(),
  connectMidiDevice: vi.fn(),
  disconnectMidiDevice: vi.fn(),
  pollMidiEvents: vi.fn(),
  setMidiLearn: vi.fn(),
}));

const mockedApi = vi.mocked(api);

describe('useMidiStore', () => {
  beforeEach(() => {
    useMidiStore.setState({
      config: null,
      devices: [],
      learnEvents: [],
      connected: false,
      loading: false,
    });
    vi.clearAllMocks();
  });

  it('initializes with empty state', () => {
    const state = useMidiStore.getState();
    expect(state.config).toBeNull();
    expect(state.devices).toEqual([]);
    expect(state.connected).toBe(false);
    expect(state.loading).toBe(false);
  });

  it('fetchConfig loads MIDI config', async () => {
    const config = {
      active_device: 'Device A',
      channel: 1,
      mappings: [],
      learn_mode: false,
      last_cc: null,
    };
    mockedApi.getMidiConfig.mockResolvedValueOnce(config);

    await useMidiStore.getState().fetchConfig();

    expect(mockedApi.getMidiConfig).toHaveBeenCalledOnce();
    expect(useMidiStore.getState().config).toEqual(config);
  });

  it('fetchDevices loads device list', async () => {
    const devices = [
      { name: 'Device A', connected: false },
      { name: 'Device B', connected: true },
    ];
    mockedApi.listMidiDevices.mockResolvedValueOnce(devices);

    await useMidiStore.getState().fetchDevices();

    expect(mockedApi.listMidiDevices).toHaveBeenCalledOnce();
    expect(useMidiStore.getState().devices).toEqual(devices);
  });

  it('connectDevice sets connected and updates config', async () => {
    const config = {
      active_device: null as string | null,
      channel: 1,
      mappings: [],
      learn_mode: false,
      last_cc: null as number | null,
    };
    useMidiStore.setState({ config });
    mockedApi.connectMidiDevice.mockResolvedValueOnce(undefined);

    await useMidiStore.getState().connectDevice('Device A');

    expect(mockedApi.connectMidiDevice).toHaveBeenCalledWith('Device A');
    expect(useMidiStore.getState().connected).toBe(true);
    expect(useMidiStore.getState().config?.active_device).toBe('Device A');
    expect(useMidiStore.getState().loading).toBe(false);
  });

  it('connectDevice throws on failure', async () => {
    mockedApi.connectMidiDevice.mockRejectedValueOnce(new Error('No device'));

    await expect(useMidiStore.getState().connectDevice('Missing')).rejects.toThrow('No device');
    expect(useMidiStore.getState().connected).toBe(false);
    expect(useMidiStore.getState().loading).toBe(false);
  });

  it('disconnectDevice clears connected state', async () => {
    const config = {
      active_device: 'Device A',
      channel: 1,
      mappings: [],
      learn_mode: false,
      last_cc: null as number | null,
    };
    useMidiStore.setState({ config, connected: true });
    mockedApi.disconnectMidiDevice.mockResolvedValueOnce(undefined);

    await useMidiStore.getState().disconnectDevice();

    expect(mockedApi.disconnectMidiDevice).toHaveBeenCalledOnce();
    expect(useMidiStore.getState().connected).toBe(false);
    expect(useMidiStore.getState().config?.active_device).toBeNull();
  });

  it('pollEvents returns events and accumulates learn events', async () => {
    const config = {
      active_device: 'Device A',
      channel: 1,
      mappings: [],
      learn_mode: true,
      last_cc: null as number | null,
    };
    useMidiStore.setState({ config });
    const events = [
      { cc: 7, channel: 1, raw_value: 64, normalized: 0.5, mapped_parameter: null, mapped_value: null },
    ];
    mockedApi.pollMidiEvents.mockResolvedValueOnce(events);

    const result = await useMidiStore.getState().pollEvents();

    expect(result).toEqual(events);
    expect(useMidiStore.getState().learnEvents).toHaveLength(1);
    expect(useMidiStore.getState().config?.last_cc).toBe(7);
  });

  it('pollEvents ignores events when not in learn mode', async () => {
    const config = {
      active_device: 'Device A',
      channel: 1,
      mappings: [],
      learn_mode: false,
      last_cc: null as number | null,
    };
    useMidiStore.setState({ config });
    const events = [
      { cc: 7, channel: 1, raw_value: 64, normalized: 0.5, mapped_parameter: null, mapped_value: null },
    ];
    mockedApi.pollMidiEvents.mockResolvedValueOnce(events);

    const result = await useMidiStore.getState().pollEvents();

    expect(result).toEqual(events);
    expect(useMidiStore.getState().learnEvents).toEqual([]);
  });

  it('pollEvents returns empty array on error', async () => {
    mockedApi.pollMidiEvents.mockRejectedValueOnce(new Error('fail'));

    const result = await useMidiStore.getState().pollEvents();

    expect(result).toEqual([]);
  });

  it('setLearn toggles learn mode in config', async () => {
    const config = {
      active_device: 'Device A',
      channel: 1,
      mappings: [],
      learn_mode: false,
      last_cc: 5,
    };
    useMidiStore.setState({ config });
    mockedApi.setMidiLearn.mockResolvedValueOnce(undefined);

    await useMidiStore.getState().setLearn(true);

    expect(mockedApi.setMidiLearn).toHaveBeenCalledWith(true);
    expect(useMidiStore.getState().config?.learn_mode).toBe(true);
  });

  it('setLearn clears learnEvents when deactivated', async () => {
    const config = {
      active_device: 'Device A',
      channel: 1,
      mappings: [],
      learn_mode: true,
      last_cc: 5,
    };
    useMidiStore.setState({ config, learnEvents: [{ cc: 1, channel: 1, raw_value: 0, normalized: 0, mapped_parameter: null, mapped_value: null }] });
    mockedApi.setMidiLearn.mockResolvedValueOnce(undefined);

    await useMidiStore.getState().setLearn(false);

    expect(useMidiStore.getState().learnEvents).toEqual([]);
  });
});
