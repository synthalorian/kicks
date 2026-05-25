import { useEffect, useRef, useCallback } from 'react';
import { useMidiStore } from '../stores/midiStore';

const PARAM_OPTIONS = [
  '-- None --',
  'Amp: Gain',
  'Amp: Bass',
  'Amp: Mid',
  'Amp: Treble',
  'Amp: Drive',
  'Amp: Master',
  'BassAmp: Gain',
  'BassAmp: Bass',
  'BassAmp: Mid',
  'BassAmp: Treble',
  'BassAmp: Drive',
  'BassAmp: Master',
  'Boost: Gain',
  'Delay: Time',
  'Delay: Feedback',
  'Delay: Mix',
  'Reverb: Size',
  'Reverb: Damping',
  'Reverb: Mix',
];

/** Map display labels to backend parameter IDs */
const PARAM_LABEL_TO_ID: Record<string, string> = {
  'Amp: Gain': 'amp.gain',
  'Amp: Bass': 'amp.bass',
  'Amp: Mid': 'amp.mid',
  'Amp: Treble': 'amp.treble',
  'Amp: Drive': 'amp.drive',
  'Amp: Master': 'amp.master',
  'BassAmp: Gain': 'bass_amp.gain',
  'BassAmp: Bass': 'bass_amp.bass',
  'BassAmp: Mid': 'bass_amp.mid',
  'BassAmp: Treble': 'bass_amp.treble',
  'BassAmp: Drive': 'bass_amp.drive',
  'BassAmp: Master': 'bass_amp.master',
  'Boost: Gain': 'boost.gain',
  'Delay: Time': 'delay.time',
  'Delay: Feedback': 'delay.feedback',
  'Delay: Mix': 'delay.mix',
  'Reverb: Size': 'reverb.size',
  'Reverb: Damping': 'reverb.damping',
  'Reverb: Mix': 'reverb.mix',
};

const PARAM_ID_TO_LABEL: Record<string, string> = Object.fromEntries(
  Object.entries(PARAM_LABEL_TO_ID).map(([label, id]) => [id, label]),
);

export function MidiConfig() {
  const {
    config,
    devices,
    connected,
    loading,
    fetchConfig,
    saveConfig,
    fetchDevices,
    connectDevice,
    disconnectDevice,
    pollEvents,
    setLearn,
  } = useMidiStore();

  const learnPollRef = useRef<ReturnType<typeof setInterval> | null>(null);

  // Fetch config + devices on mount
  useEffect(() => {
    fetchConfig();
    fetchDevices();
  }, [fetchConfig, fetchDevices]);

  // Poll MIDI events while learn mode is active
  useEffect(() => {
    if (config?.learn_mode) {
      learnPollRef.current = setInterval(() => {
        pollEvents();
      }, 100);
    } else {
      if (learnPollRef.current) {
        clearInterval(learnPollRef.current);
        learnPollRef.current = null;
      }
    }
    return () => {
      if (learnPollRef.current) {
        clearInterval(learnPollRef.current);
        learnPollRef.current = null;
      }
    };
  }, [config?.learn_mode, pollEvents]);

  const handleConnect = useCallback(
    async (deviceName: string) => {
      try {
        await connectDevice(deviceName);
      } catch {
        // Error handled in store
      }
    },
    [connectDevice],
  );

  const handleDisconnect = useCallback(async () => {
    await disconnectDevice();
  }, [disconnectDevice]);

  const toggleLearn = useCallback(async () => {
    if (!config) return;
    const newState = !config.learn_mode;
    await setLearn(newState);
  }, [config, setLearn]);

  const updateMapping = useCallback(
    (index: number, field: string, value: number | string) => {
      if (!config) return;
      const mappings = [...config.mappings];
      const mapping = { ...mappings[index] } as Record<string, unknown>;

      if (field === 'cc' || field === 'channel' || field === 'min' || field === 'max') {
        mapping[field] = Number(value);
      } else if (field === 'parameter_id') {
        const label = value as string;
        if (label === '-- None --') {
          mapping.parameter_id = '';
          mapping.label = '';
        } else {
          mapping.parameter_id = PARAM_LABEL_TO_ID[label] ?? '';
          mapping.label = label;
        }
      }

      mappings[index] = mapping as unknown as typeof config.mappings[number];
      saveConfig({ ...config, mappings });
    },
    [config, saveConfig],
  );

  const addMapping = useCallback(() => {
    if (!config) return;
    const cc = config.last_cc ?? (config.mappings.length + 1);
    const mappings = [
      ...config.mappings,
      { cc, channel: config.channel, parameter_id: '', label: '', min: 0, max: 127 },
    ];
    saveConfig({ ...config, mappings, last_cc: null });
  }, [config, saveConfig]);

  const removeMapping = useCallback(
    (index: number) => {
      if (!config) return;
      const mappings = config.mappings.filter((_, i) => i !== index);
      saveConfig({ ...config, mappings });
    },
    [config, saveConfig],
  );

  if (!config) {
    return (
      <div className="flex items-center justify-center h-48">
        <span className="text-[var(--text-muted)] text-sm">Loading MIDI config...</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-[var(--text-h)]">MIDI Config</h2>
          <p className="text-sm text-[var(--text-muted)] mt-1">
            Map MIDI controllers to amp parameters.
          </p>
        </div>
        <button
          onClick={toggleLearn}
          className={`px-4 py-2 rounded-lg text-sm font-medium transition-colors cursor-pointer ${
            config.learn_mode
              ? 'bg-red-700 text-white animate-pulse'
              : 'bg-[var(--accent)] text-white hover:opacity-90'
          }`}
        >
          {config.learn_mode ? 'MIDI Learn Active' : 'MIDI Learn'}
        </button>
      </div>

      {/* Device selection */}
      <div className="flex flex-wrap items-center gap-3">
        <span className="text-sm text-[var(--text)] font-medium">MIDI Device:</span>
        {connected ? (
          <div className="flex items-center gap-2">
            <span className="text-sm text-green-400 font-medium">{config.active_device}</span>
            <button
              onClick={handleDisconnect}
              disabled={loading}
              className="px-3 py-1.5 rounded-lg bg-red-700/30 text-red-400 text-xs font-medium hover:bg-red-700/50 transition-colors cursor-pointer disabled:opacity-50"
            >
              Disconnect
            </button>
          </div>
        ) : (
          <div className="flex flex-wrap gap-2">
            {devices.length === 0 ? (
              <span className="text-xs text-[var(--text-muted)]">No MIDI devices found</span>
            ) : (
              devices.map((dev) => (
                <button
                  key={dev.name}
                  onClick={() => handleConnect(dev.name)}
                  disabled={loading}
                  className="px-3 py-1.5 rounded-lg border border-[var(--border)] text-xs font-medium
                    text-[var(--text-muted)] hover:text-[var(--text)] hover:border-zinc-600
                    transition-colors cursor-pointer disabled:opacity-50"
                >
                  {dev.name}
                </button>
              ))
            )}
            <button
              onClick={() => fetchDevices()}
              className="px-3 py-1.5 rounded-lg text-xs text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
            >
              ↻ Refresh
            </button>
          </div>
        )}
      </div>

      {/* MIDI Channel */}
      <div className="flex items-center gap-3">
        <span className="text-sm text-[var(--text)] font-medium">MIDI Channel:</span>
        {[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].map((ch) => (
          <button
            key={ch}
            onClick={() => saveConfig({ ...config, channel: ch })}
            className={`w-9 h-9 rounded text-xs font-medium transition-colors cursor-pointer ${
              config.channel === ch
                ? 'bg-[var(--accent)] text-white'
                : 'bg-[var(--bg-surface)] text-[var(--text-muted)] border border-[var(--border)] hover:text-[var(--text)]'
            }`}
          >
            {ch === 0 ? 'All' : ch}
          </button>
        ))}
      </div>

      {/* Learn mode banner */}
      {config.learn_mode && (
        <div className="border border-yellow-700 rounded-xl p-4 bg-yellow-900/20 text-yellow-400 text-sm space-y-2">
          <p>MIDI Learn active. Move a controller on your MIDI device to assign it.</p>
          {config.last_cc !== null && (
            <div className="flex items-center gap-3 text-sm">
              <span className="text-yellow-300 font-medium">
                Captured CC #{config.last_cc}
              </span>
              <button
                onClick={addMapping}
                className="px-3 py-1 rounded bg-green-700 text-white text-xs font-medium hover:bg-green-600 transition-colors cursor-pointer"
              >
                Add Mapping +
              </button>
            </div>
          )}
        </div>
      )}

      {/* Mapping table */}
      <div className="border border-[var(--border)] rounded-xl overflow-hidden">
        <table className="w-full text-sm">
          <thead>
            <tr className="bg-[var(--bg-surface)] text-[var(--text-muted)] text-xs uppercase tracking-wider">
              <th className="text-left px-4 py-3 font-medium">CC</th>
              <th className="text-left px-4 py-3 font-medium">Channel</th>
              <th className="text-left px-4 py-3 font-medium">Parameter</th>
              <th className="text-left px-4 py-3 font-medium">Min</th>
              <th className="text-left px-4 py-3 font-medium">Max</th>
              <th className="text-right px-4 py-3 font-medium">Actions</th>
            </tr>
          </thead>
          <tbody>
            {config.mappings.length === 0 ? (
              <tr>
                <td colSpan={6} className="px-4 py-8 text-center text-[var(--text-muted)] text-xs">
                  {config.learn_mode
                    ? 'Move a MIDI controller to capture it'
                    : 'No mappings yet. Enable MIDI Learn or add one manually.'}
                </td>
              </tr>
            ) : (
              config.mappings.map((row, i) => (
                <tr
                  key={i}
                  className="border-t border-[var(--border)] hover:bg-[var(--surface-hover)]"
                >
                  <td className="px-4 py-2">
                    <input
                      type="number"
                      min={0}
                      max={127}
                      value={row.cc}
                      onChange={(e) => updateMapping(i, 'cc', parseInt(e.target.value) || 0)}
                      className="w-16 px-2 py-1 rounded border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-xs outline-none focus:border-[var(--accent)]"
                    />
                  </td>
                  <td className="px-4 py-2">
                    <select
                      value={row.channel}
                      onChange={(e) => updateMapping(i, 'channel', parseInt(e.target.value))}
                      className="px-2 py-1 rounded border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-xs outline-none focus:border-[var(--accent)]"
                    >
                      {[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16].map((ch) => (
                        <option key={ch} value={ch}>
                          {ch === 0 ? 'All' : ch}
                        </option>
                      ))}
                    </select>
                  </td>
                  <td className="px-4 py-2">
                    <select
                      value={PARAM_ID_TO_LABEL[row.parameter_id] || '-- None --'}
                      onChange={(e) => updateMapping(i, 'parameter_id', e.target.value)}
                      className="px-2 py-1 rounded border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-xs outline-none focus:border-[var(--accent)] max-w-44"
                    >
                      {PARAM_OPTIONS.map((opt) => (
                        <option key={opt} value={opt}>
                          {opt}
                        </option>
                      ))}
                    </select>
                  </td>
                  <td className="px-4 py-2">
                    <input
                      type="number"
                      min={0}
                      max={127}
                      value={row.min}
                      onChange={(e) => updateMapping(i, 'min', parseInt(e.target.value) || 0)}
                      className="w-16 px-2 py-1 rounded border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-xs outline-none focus:border-[var(--accent)]"
                    />
                  </td>
                  <td className="px-4 py-2">
                    <input
                      type="number"
                      min={0}
                      max={127}
                      value={row.max}
                      onChange={(e) => updateMapping(i, 'max', parseInt(e.target.value) || 127)}
                      className="w-16 px-2 py-1 rounded border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-xs outline-none focus:border-[var(--accent)]"
                    />
                  </td>
                  <td className="px-4 py-2 text-right">
                    <button
                      onClick={() => removeMapping(i)}
                      className="text-xs text-red-400 hover:text-red-300 transition-colors cursor-pointer px-2 py-1"
                    >
                      ✕
                    </button>
                  </td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </div>

      {/* Add row button */}
      <button
        onClick={addMapping}
        className="self-start px-4 py-2 rounded-lg border border-dashed border-[var(--border)] text-sm text-[var(--text-muted)] hover:text-[var(--text)] hover:border-zinc-600 transition-colors cursor-pointer"
      >
        + Add Mapping
      </button>
    </div>
  );
}
