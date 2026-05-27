import { useEffect, useState, useCallback } from 'react';
import { usePresetsStore } from '../stores/presetsStore';
import type { BankDescriptor } from '../types/tauri';
import { useHotkeys } from '../hooks/useHotkeys';

export function Presets() {
  const { banks, loading, fetchPresets, savePreset, loadPreset, deletePreset, renamePreset } =
    usePresetsStore();
  const [activeBank, setActiveBank] = useState<string | null>(null);
  const [search, setSearch] = useState('');
  const [showSave, setShowSave] = useState(false);
  const [saveName, setSaveName] = useState('');
  const [renaming, setRenaming] = useState<string | null>(null);
  const [renameValue, setRenameValue] = useState('');

  useEffect(() => {
    fetchPresets();
  }, [fetchPresets]);

  const effectiveActiveBank = activeBank ?? banks[0]?.name ?? null;
  const activeBankData: BankDescriptor | undefined = banks.find((b) => b.name === effectiveActiveBank);

  const filteredPresets = (activeBankData?.presets ?? []).filter(
    (p) =>
      p.name.toLowerCase().includes(search.toLowerCase()) ||
      p.tags.some((t) => t.toLowerCase().includes(search.toLowerCase())),
  );

  const handleSave = useCallback(async () => {
    if (!saveName.trim()) return;
    await savePreset(effectiveActiveBank ?? 'Default', saveName.trim());
    setSaveName('');
    setShowSave(false);
  }, [saveName, effectiveActiveBank, savePreset]);

  const handleLoad = useCallback(
    async (bankName: string, presetName: string) => {
      await loadPreset(bankName, presetName);
    },
    [loadPreset],
  );

  const handleDelete = useCallback(
    async (bankName: string, presetName: string) => {
      if (window.confirm(`Delete preset "${presetName}"?`)) {
        await deletePreset(bankName, presetName);
      }
    },
    [deletePreset],
  );

  const handleRename = useCallback(
    async (bankName: string, oldName: string) => {
      if (renameValue.trim() && renameValue !== oldName) {
        await renamePreset(bankName, oldName, renameValue.trim());
      }
      setRenaming(null);
      setRenameValue('');
    },
    [renameValue, renamePreset],
  );

  useHotkeys([
    { key: 'n', ctrl: true, handler: () => setShowSave((s) => !s), ignoreInput: true },
  ]);

  return (
    <div className="flex flex-col gap-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-[var(--text-h)]">Presets</h2>
          <p className="text-sm text-[var(--text-muted)] mt-1">
            Browse, load, and manage your tone presets.
          </p>
        </div>
        <button
          onClick={() => setShowSave(!showSave)}
          className="px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity cursor-pointer"
        >
          Save Current
        </button>
      </div>

      {/* Save dialog */}
      {showSave && (
        <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)] flex items-center gap-3">
          <input
            type="text"
            value={saveName}
            onChange={(e) => setSaveName(e.target.value)}
            placeholder="Preset name..."
            className="flex-1 px-3 py-2 rounded-lg border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-sm outline-none focus:border-[var(--accent)]"
            autoFocus
            onKeyDown={(e) => e.key === 'Enter' && handleSave()}
          />
          <select
            value={effectiveActiveBank ?? ''}
            onChange={(e) => setActiveBank(e.target.value)}
            className="px-3 py-2 rounded-lg border border-[var(--border)] bg-[var(--bg)] text-[var(--text)] text-sm outline-none focus:border-[var(--accent)]"
          >
            {banks.map((b) => (
              <option key={b.name} value={b.name}>
                {b.name}
              </option>
            ))}
            {banks.length === 0 && <option value="Default">Default</option>}
          </select>
          <button
            onClick={handleSave}
            disabled={!saveName.trim()}
            className="px-4 py-2 rounded-lg bg-green-700 text-white text-sm font-medium hover:bg-green-600 transition-colors disabled:opacity-40 cursor-pointer"
          >
            Save
          </button>
          <button
            onClick={() => setShowSave(false)}
            className="px-3 py-2 text-sm text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
          >
            Cancel
          </button>
        </div>
      )}

      {/* Bank tabs + search */}
      <div className="flex items-center gap-3 flex-wrap">
        {banks.map((b) => (
          <button
            key={b.name}
            onClick={() => setActiveBank(b.name)}
            className={`px-3 py-1.5 rounded-lg text-sm font-medium transition-colors cursor-pointer ${
              effectiveActiveBank === b.name
                ? 'bg-[var(--accent)] text-white'
                : 'bg-[var(--bg-surface)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)]'
            }`}
          >
            {b.name}
            <span className="ml-1.5 text-xs opacity-60">({b.presets.length})</span>
          </button>
        ))}
        <div className="flex-1" />
        <input
          type="text"
          value={search}
          onChange={(e) => setSearch(e.target.value)}
          placeholder="Search presets..."
          className="px-3 py-1.5 rounded-lg border border-[var(--border)] bg-[var(--bg-surface)] text-[var(--text)] text-sm outline-none focus:border-[var(--accent)] w-48"
        />
      </div>

      {/* Preset grid */}
      {loading ? (
        <div className="text-[var(--text-muted)] text-sm">Loading presets...</div>
      ) : filteredPresets.length === 0 ? (
        <div className="border border-dashed border-[var(--border)] rounded-xl p-8 text-center text-[var(--text-muted)] text-sm">
          {search ? 'No presets match your search.' : 'No presets yet. Save your first tone!'}
        </div>
      ) : (
        <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
          {filteredPresets.map((preset) => (
            <div
              key={preset.name}
              className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)] flex flex-col gap-2 hover:border-zinc-600 transition-colors group"
            >
              {renaming === preset.name ? (
                <input
                  type="text"
                  value={renameValue}
                  onChange={(e) => setRenameValue(e.target.value)}
                  onBlur={() => { if (effectiveActiveBank) handleRename(effectiveActiveBank, preset.name); }}
                    onKeyDown={(e) => {
                      if (e.key === 'Enter' && effectiveActiveBank) handleRename(effectiveActiveBank, preset.name);
                      if (e.key === 'Escape') setRenaming(null);
                    }}
                  className="px-2 py-0.5 rounded border border-[var(--accent)] bg-[var(--bg)] text-[var(--text)] text-sm outline-none"
                  autoFocus
                />
              ) : (
                <span className="text-[var(--text)] font-medium text-sm truncate">
                  {preset.name}
                </span>
              )}

              {preset.description && (
                <span className="text-xs text-[var(--text-muted)] line-clamp-2">
                  {preset.description}
                </span>
              )}

              {preset.tags.length > 0 && (
                <div className="flex gap-1 flex-wrap mt-1">
                  {preset.tags.map((tag) => (
                    <span
                      key={tag}
                      className="text-[10px] px-1.5 py-0.5 rounded-full bg-[var(--bg-elevated)] text-[var(--text-muted)]"
                    >
                      {tag}
                    </span>
                  ))}
                </div>
              )}

              <div className="text-[10px] text-[var(--text-muted)] mt-1">
                {preset.modified}
              </div>

              {/* Actions */}
              <div className="flex gap-2 mt-2 opacity-0 group-hover:opacity-100 transition-opacity">
                <button
                  onClick={() => { if (effectiveActiveBank) handleLoad(effectiveActiveBank, preset.name); }}
                  className="flex-1 px-2 py-1 rounded text-xs bg-[var(--accent)] text-white hover:opacity-90 transition-opacity cursor-pointer"
                >
                  Load
                </button>
                <button
                  onClick={() => {
                    setRenaming(preset.name);
                    setRenameValue(preset.name);
                  }}
                  className="px-2 py-1 rounded text-xs border border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
                >
                  Rename
                </button>
                <button
                  onClick={() => { if (effectiveActiveBank) handleDelete(effectiveActiveBank, preset.name); }}
                  className="px-2 py-1 rounded text-xs border border-red-800 text-red-400 hover:bg-red-900/30 transition-colors cursor-pointer"
                >
                  Delete
                </button>
              </div>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
