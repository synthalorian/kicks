import { useState, useEffect, useMemo } from 'react';
import type { AmpPresetInfo } from '../../types/tauri';
import * as api from '../../lib/tauri';

interface AmpPresetSelectorProps {
  onApplyPreset: (presetName: string) => void;
  onClose: () => void;
}

/** Tag-based category filter chips. */
const CATEGORIES = [
  { key: 'all', label: 'All' },
  { key: 'clean', label: 'Clean' },
  { key: 'crunch', label: 'Crunch' },
  { key: 'lead', label: 'Lead' },
  { key: 'high-gain', label: 'High Gain' },
  { key: 'bass', label: 'Bass' },
  { key: 'specialty', label: 'Specialty' },
] as const;

type CategoryKey = (typeof CATEGORIES)[number]['key'];

function fmtBar(value: number, size: number = 6): string {
  const filled = Math.round(value * size);
  return '█'.repeat(filled) + '░'.repeat(size - filled);
}

export function AmpPresetSelector({ onApplyPreset, onClose }: AmpPresetSelectorProps) {
  const [presets, setPresets] = useState<AmpPresetInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [search, setSearch] = useState('');
  const [category, setCategory] = useState<CategoryKey>('all');
  const [applying, setApplying] = useState<string | null>(null);

  useEffect(() => {
    api.listAmpPresets().then((data) => {
      setPresets(data);
      setLoading(false);
    }).catch(() => setLoading(false));
  }, []);

  const filtered = useMemo(() => {
    let list = presets;
    if (category !== 'all') {
      list = list.filter((p) => p.tags.includes(category));
    }
    if (search.trim()) {
      const q = search.toLowerCase();
      list = list.filter(
        (p) =>
          p.name.toLowerCase().includes(q) ||
          p.description.toLowerCase().includes(q) ||
          p.tags.some((t) => t.toLowerCase().includes(q)),
      );
    }
    return list;
  }, [presets, category, search]);

  const handleApply = async (preset: AmpPresetInfo) => {
    setApplying(preset.name);
    try {
      await api.applyAmpPreset(preset.name);
      onApplyPreset(preset.name);
    } catch (err) {
      console.error('Failed to apply amp preset:', err);
    } finally {
      setApplying(null);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/60" onClick={onClose}>
      <div
        className="w-full max-w-3xl max-h-[85vh] bg-[var(--bg-surface)] rounded-xl border border-[var(--border)] shadow-2xl flex flex-col overflow-hidden"
        onClick={(e) => e.stopPropagation()}
      >
        {/* Header */}
        <div className="flex items-center justify-between px-5 py-4 border-b border-[var(--border)]">
          <div>
            <h3 className="text-lg font-bold text-[var(--text-h)]">Amp Presets</h3>
            <p className="text-xs text-[var(--text-muted)] mt-0.5">
              Browse {presets.length} built-in amp tones. Click to apply to your Amp slot.
            </p>
          </div>
          <button
            onClick={onClose}
            className="w-8 h-8 rounded-lg bg-[var(--bg-elevated)] border border-[var(--border)] flex items-center justify-center text-sm text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
          >
            ✕
          </button>
        </div>

        {/* Search + filter */}
        <div className="px-5 py-3 border-b border-[var(--border)] space-y-3">
          <input
            type="text"
            value={search}
            onChange={(e) => setSearch(e.target.value)}
            placeholder="Search presets by name, description, or tags..."
            className="w-full px-3 py-2 rounded-lg bg-[var(--bg-elevated)] border border-[var(--border)] text-sm text-[var(--text)] placeholder:text-[var(--text-muted)] outline-none focus:border-[var(--accent)] transition-colors"
            autoFocus
          />
          <div className="flex items-center gap-1.5 flex-wrap">
            {CATEGORIES.map((cat) => (
              <button
                key={cat.key}
                onClick={() => setCategory(cat.key)}
                className={`px-3 py-1 rounded-full text-xs font-medium transition-colors cursor-pointer ${
                  category === cat.key
                    ? 'bg-[var(--accent)] text-white'
                    : 'bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-[var(--text)] border border-[var(--border)]'
                }`}
              >
                {cat.label}
              </button>
            ))}
          </div>
        </div>

        {/* Preset grid */}
        <div className="flex-1 overflow-y-auto p-5">
          {loading ? (
            <div className="flex items-center justify-center h-32">
              <span className="text-sm text-[var(--text-muted)]">Loading presets...</span>
            </div>
          ) : filtered.length === 0 ? (
            <div className="flex flex-col items-center justify-center h-32 gap-2">
              <span className="text-sm text-[var(--text-muted)]">No presets match your search.</span>
              <button
                onClick={() => { setSearch(''); setCategory('all'); }}
                className="text-xs text-[var(--accent)] hover:underline cursor-pointer"
              >
                Clear filters
              </button>
            </div>
          ) : (
            <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
              {filtered.map((preset) => (
                  <div
                    key={preset.name}
                    className="rounded-lg border border-[var(--border)] bg-[var(--bg-elevated)] p-4 hover:border-zinc-500 transition-colors"
                  >
                    <div className="flex items-start justify-between gap-2">
                      <div className="min-w-0">
                        <span className="text-sm font-bold text-[var(--text)] block truncate">
                          {preset.name}
                        </span>
                        <span className="text-[10px] text-[var(--text-muted)] block mt-0.5 line-clamp-2">
                          {preset.description}
                        </span>
                      </div>
                      <button
                        onClick={() => handleApply(preset)}
                        disabled={applying === preset.name}
                        className="shrink-0 px-3 py-1 rounded-md bg-[var(--accent)] text-white text-xs font-medium hover:opacity-90 transition-opacity disabled:opacity-40 cursor-pointer"
                      >
                        {applying === preset.name ? '...' : 'Apply'}
                      </button>
                    </div>

                    {/* Parameter bars */}
                    <div className="mt-3 grid grid-cols-3 gap-x-3 gap-y-1">
                      {(['gain', 'drive', 'master'] as const).map((key) => {
                        const val = key === 'gain' ? preset.gain : key === 'drive' ? preset.drive : preset.master;
                        const labels: Record<string, string> = { gain: 'Gain', drive: 'Drive', master: 'Vol' };
                        return (
                          <div key={key} className="flex items-center gap-1">
                            <span className="text-[9px] text-[var(--text-muted)] uppercase w-6 shrink-0">{labels[key]}</span>
                            <span className="text-[10px] font-mono text-[var(--text)] tabular-nums">
                              {fmtBar(val, 4)}
                            </span>
                          </div>
                        );
                      })}
                    </div>

                    {/* EQ summary */}
                    <div className="mt-1.5 flex items-center gap-2 text-[9px] text-[var(--text-muted)]">
                      <span>B: {(preset.bass * 10).toFixed(0)}</span>
                      <span>M: {(preset.mid * 10).toFixed(0)}</span>
                      <span>T: {(preset.treble * 10).toFixed(0)}</span>
                      <span className="flex-1" />
                      {preset.tags.slice(0, 3).map((tag) => (
                        <span key={tag} className="px-1.5 py-0.5 rounded bg-black/20 text-[9px] text-[var(--text-muted)]">
                          {tag}
                        </span>
                      ))}
                    </div>
                  </div>
                ))}
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="px-5 py-3 border-t border-[var(--border)] flex items-center justify-between">
          <span className="text-[10px] text-[var(--text-muted)]">
            {filtered.length} of {presets.length} presets shown
          </span>
          <button
            onClick={onClose}
            className="px-4 py-1.5 rounded-lg border border-[var(--border)] text-xs text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
