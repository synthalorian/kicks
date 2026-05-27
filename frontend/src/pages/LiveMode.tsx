import { useEffect, useState, useCallback } from 'react';
import * as api from '../lib/tauri';
import type { SceneInfo, ChainSlot } from '../types/tauri';
import { useHotkeys } from '../hooks/useHotkeys';

// ── Plugin parameter metadata ──

interface PluginMeta { label: string; color: string; params: { id: string; label: string }[] }

const PLUGIN_META: Record<string, PluginMeta> = {
  Input:  { label: 'IN',  color: 'bg-zinc-600', params: [] },
  Output: { label: 'OUT', color: 'bg-zinc-600', params: [{ id: 'volume', label: 'Vol' }] },
  Boost:  { label: 'BST', color: 'bg-amber-700', params: [{ id: 'gain', label: 'Gain' }] },
  Amp:    { label: 'AMP', color: 'bg-red-700', params: [
    { id: 'gain', label: 'Gain' }, { id: 'bass', label: 'Bass' }, { id: 'mid', label: 'Mid' },
    { id: 'treble', label: 'Treble' }, { id: 'drive', label: 'Drive' }, { id: 'master', label: 'Master' },
  ]},
  BassAmp: { label: 'BASS', color: 'bg-indigo-700', params: [
    { id: 'gain', label: 'Gain' }, { id: 'bass', label: 'Bass' }, { id: 'mid', label: 'Mid' },
    { id: 'treble', label: 'Treble' }, { id: 'drive', label: 'Drive' }, { id: 'master', label: 'Master' },
  ]},
  Cab:    { label: 'CAB', color: 'bg-stone-600', params: [
    { id: 'level', label: 'Level' }, { id: 'low_cut', label: 'Lo Cut' }, { id: 'high_cut', label: 'Hi Cut' },
  ]},
  Delay:  { label: 'DLY', color: 'bg-cyan-700', params: [
    { id: 'time', label: 'Time' }, { id: 'feedback', label: 'Fdbk' }, { id: 'mix', label: 'Mix' },
  ]},
  Reverb: { label: 'REV', color: 'bg-sky-700', params: [
    { id: 'size', label: 'Size' }, { id: 'damping', label: 'Damp' }, { id: 'mix', label: 'Mix' },
  ]},
};

function SceneDetailView({ sceneName, sceneIndex, slotCount }: { sceneName: string; sceneIndex: number; slotCount: number }) {
  const [slots, setSlots] = useState<ChainSlot[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    api.getScene(sceneIndex).then((detail) => {
      setSlots(detail.signal_chain.slots);
      setLoading(false);
    }).catch(() => setLoading(false));
  }, [sceneIndex]);

  const activeSlots = slots.filter((s) => s.enabled && s.plugin_type !== 'Input' && s.plugin_type !== 'Output');

  return (
    <div className="border border-[var(--border)] rounded-xl bg-[var(--bg-surface)] overflow-hidden">
      <div className="bg-gradient-to-r from-zinc-800 to-zinc-900 px-6 py-5 border-b border-[var(--border)]">
        <div className="flex items-center justify-between">
          <div className="flex items-center gap-4">
            <span className="text-3xl font-black text-[var(--text-h)] tracking-tight">{sceneName}</span>
            <span className="text-xs text-[var(--text-muted)] bg-black/30 px-2 py-1 rounded-md">
              Scene {sceneIndex + 1}
            </span>
          </div>
          <span className="text-xs text-[var(--text-muted)] bg-black/30 px-2 py-1 rounded-md">
            {slotCount} slots · {activeSlots.length} active
          </span>
        </div>
      </div>

      {loading ? (
        <div className="flex items-center justify-center h-32">
          <span className="text-sm text-[var(--text-muted)]">Loading scene detail...</span>
        </div>
      ) : (
        <>
          <div className="p-6 grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-3">
            {slots.map((slot) => {
              const meta = PLUGIN_META[slot.plugin_type];
              const bgColor = meta?.color ?? 'bg-zinc-600';
              const isFixed = slot.plugin_type === 'Input' || slot.plugin_type === 'Output';

              return (
                <div
                  key={slot.id}
                  className={`rounded-xl border ${
                    slot.enabled
                      ? 'border-[var(--border)] bg-[var(--bg-elevated)]'
                      : 'border-dashed border-zinc-700 bg-[var(--bg-elevated)]/60 opacity-50'
                  } overflow-hidden`}
                >
                  <div className={`flex items-center gap-2 px-4 py-2 ${bgColor}`}>
                    <span className="text-sm font-bold text-white tracking-wider">{meta?.label ?? slot.plugin_type}</span>
                    <span className="flex-1 text-xs text-white/70 truncate">{slot.id}</span>
                    {!slot.enabled && <span className="text-[10px] text-white/50 font-medium">OFF</span>}
                    {slot.enabled && slot.wet_dry < 1.0 && (
                      <span className="text-[10px] text-white/70">{(slot.wet_dry * 100).toFixed(0)}%</span>
                    )}
                  </div>

                  <div className="p-4 space-y-3">
                    {meta ? meta.params.map((p) => {
                      const val = slot.parameters[p.id];
                      if (val === undefined) return null;
                      return (
                        <div key={p.id}>
                          <div className="flex items-center justify-between mb-1">
                            <span className="text-[10px] uppercase text-[var(--text-muted)] font-medium tracking-wider">
                              {p.label}
                            </span>
                            <span className="text-sm font-bold text-[var(--text)] tabular-nums">
                              {(val * 100).toFixed(0)}%
                            </span>
                          </div>
                          <div className="h-2 rounded-full bg-black/30 overflow-hidden">
                            <div
                              className={`h-full rounded-full transition-all duration-200 ${
                                val > 0.7 ? 'bg-red-500' : val > 0.4 ? 'bg-amber-500' : 'bg-green-500'
                              }`}
                              style={{ width: `${val * 100}%` }}
                            />
                          </div>
                        </div>
                      );
                    }) : (
                      <div className="text-[10px] text-[var(--text-muted)] text-center py-2">
                        {isFixed ? 'Fixed slot' : 'No parameters'}
                      </div>
                    )}
                  </div>
                </div>
              );
            })}
          </div>

          <div className="px-6 py-3 border-t border-[var(--border)] bg-black/10 text-center">
            <span className="text-[10px] text-[var(--text-muted)]">
              Arrow keys to navigate · Active scene: {sceneName}
            </span>
          </div>
        </>
      )}
    </div>
  );
}

export function LiveMode() {
  const [scenes, setScenes] = useState<SceneInfo[]>([]);
  const [activeScene, setActiveScene] = useState<number | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [newSceneName, setNewSceneName] = useState('');
  const [renamingIdx, setRenamingIdx] = useState<number | null>(null);
  const [renameValue, setRenameValue] = useState('');
  const [confirmDelete, setConfirmDelete] = useState<number | null>(null);
  const [error, setError] = useState<string | null>(null);

  const loadScenes = async () => {
    setLoading(true);
    setError(null);
    try {
      const result = await api.listScenes();
      setScenes(result);
      const active = result.find((s) => s.is_active);
      setActiveScene(active?.index ?? null);
    } catch (err) {
      setError('Failed to load scenes');
      console.error('Failed to load scenes:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleSelectScene = useCallback(async (index: number) => {
    setError(null);
    try {
      await api.loadScene(index);
      setActiveScene(index);
      setScenes((prev) =>
        prev.map((s) => ({ ...s, is_active: s.index === index })),
      );
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to load scene');
      console.error('Failed to load scene:', err);
    }
  }, []);

  const handleSaveScene = useCallback(async () => {
    const name = newSceneName.trim();
    if (!name) return;
    setSaving(true);
    setError(null);
    try {
      const info = await api.saveScene(name);
      setScenes((prev) => [...prev, info]);
      setShowSaveDialog(false);
      setNewSceneName('');
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to save scene');
    } finally {
      setSaving(false);
    }
  }, [newSceneName]);

  const handleOverwriteScene = useCallback(async (index: number) => {
    setError(null);
    try {
      await api.updateScene(index);
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to update scene');
    }
  }, []);

  const handleDeleteScene = useCallback(async (index: number) => {
    setError(null);
    try {
      await api.deleteScene(index);
      setScenes((prev) => prev.filter((s) => s.index !== index));
      setActiveScene((prev) => {
        if (prev === index) return scenes.length > 1 ? 0 : null;
        if (prev !== null && prev > index) return prev - 1;
        return prev;
      });
      setConfirmDelete(null);
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to delete scene');
    }
  }, [scenes.length]);

  const handleRenameScene = useCallback(async (index: number) => {
    const newName = renameValue.trim();
    if (!newName) {
      setRenamingIdx(null);
      return;
    }
    setError(null);
    try {
      await api.renameScene(index, newName);
      setScenes((prev) =>
        prev.map((s) => (s.index === index ? { ...s, name: newName } : s)),
      );
      setRenamingIdx(null);
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to rename scene');
    }
  }, [renameValue]);

  const handleNextScene = useCallback(async () => {
    setError(null);
    try {
      const info = await api.nextScene();
      if (info) {
        setActiveScene(info.index);
        setScenes((prev) =>
          prev.map((s) => ({ ...s, is_active: s.index === info!.index })),
        );
      }
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to switch scene');
    }
  }, []);

  const handlePrevScene = useCallback(async () => {
    setError(null);
    try {
      const info = await api.prevScene();
      if (info) {
        setActiveScene(info.index);
        setScenes((prev) =>
          prev.map((s) => ({ ...s, is_active: s.index === info!.index })),
        );
      }
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to switch scene');
    }
  }, []);

  const handleReorder = useCallback(async (from: number, to: number) => {
    setError(null);
    try {
      await api.reorderScene(from, to);
      await loadScenes();
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to reorder scenes');
    }
  }, []);

  useEffect(() => {
    (async () => {
      try {
        const result = await api.listScenes();
        setScenes(result);
        const active = result.find((s) => s.is_active);
        setActiveScene(active?.index ?? null);
      } catch (err) {
        setError('Failed to load scenes');
        console.error('Failed to load scenes:', err);
      } finally {
        setLoading(false);
      }
    })();
  }, []);

  useHotkeys([
    { key: 'ArrowRight', handler: handleNextScene, ignoreInput: true },
    { key: 'ArrowDown', handler: handleNextScene, ignoreInput: true },
    { key: 'ArrowLeft', handler: handlePrevScene, ignoreInput: true },
    { key: 'ArrowUp', handler: handlePrevScene, ignoreInput: true },
  ]);

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <span className="text-[var(--text-muted)] text-sm">Loading scenes...</span>
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-[var(--text-h)]">Live Mode</h2>
          <p className="text-sm text-[var(--text-muted)] mt-1">
            Performance view with scene grid and large controls.
          </p>
        </div>
        <div className="flex items-center gap-2">
          <span className="text-xs text-[var(--text-muted)] bg-[var(--bg-surface)] px-3 py-1.5 rounded-lg border border-[var(--border)]">
            {scenes.length} scene{scenes.length !== 1 ? 's' : ''}
          </span>
          <button
            onClick={() => setShowSaveDialog(true)}
            className="px-3 py-1.5 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity cursor-pointer"
          >
            + Save Scene
          </button>
        </div>
      </div>

      {/* Error */}
      {error && (
        <div className="border border-red-500/30 bg-red-500/10 rounded-xl p-3 text-sm text-red-400">
          {error}
        </div>
      )}

      {/* Save dialog */}
      {showSaveDialog && (
        <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
          <div className="flex items-center gap-3">
            <input
              type="text"
              value={newSceneName}
              onChange={(e) => setNewSceneName(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === 'Enter') handleSaveScene();
                if (e.key === 'Escape') setShowSaveDialog(false);
              }}
              placeholder="Scene name..."
              className="flex-1 px-3 py-2 rounded-lg bg-[var(--bg-elevated)] border border-[var(--border)] text-sm text-[var(--text)] placeholder:text-[var(--text-muted)] outline-none focus:border-[var(--accent)] transition-colors"
              autoFocus
            />
            <button
              onClick={handleSaveScene}
              disabled={saving || !newSceneName.trim()}
              className="px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity disabled:opacity-40 cursor-pointer"
            >
              {saving ? 'Saving...' : 'Save'}
            </button>
            <button
              onClick={() => setShowSaveDialog(false)}
              className="px-3 py-2 rounded-lg border border-[var(--border)] text-sm text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
            >
              Cancel
            </button>
          </div>
        </div>
      )}

      {/* Empty state */}
      {!loading && scenes.length === 0 && (
        <div className="border-2 border-dashed border-[var(--border)] rounded-xl p-12 flex flex-col items-center justify-center gap-3 bg-[var(--bg-surface)]">
          <span className="text-[var(--text-muted)] text-sm">
            No scenes yet. Save your current signal chain as a scene to get started.
          </span>
        </div>
      )}

      {/* Scene grid */}
      {scenes.length > 0 && (
        <>
          <div className="grid grid-cols-2 sm:grid-cols-3 lg:grid-cols-4 gap-3">
            {scenes.map((scene) => {
              const isActive = activeScene === scene.index;
              const isConfirming = confirmDelete === scene.index;
              const isRenaming = renamingIdx === scene.index;

              return (
                <div
                  key={scene.index}
                  className={`rounded-xl p-4 text-left transition-all ${
                    isActive
                      ? 'border-2 border-[var(--accent)] bg-[var(--accent-bg)]'
                      : 'border border-[var(--border)] bg-[var(--bg-surface)] hover:border-zinc-600'
                  }`}
                >
                  {isRenaming ? (
                    <input
                      type="text"
                      value={renameValue}
                      onChange={(e) => setRenameValue(e.target.value)}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter') handleRenameScene(scene.index);
                        if (e.key === 'Escape') setRenamingIdx(null);
                      }}
                      onBlur={() => handleRenameScene(scene.index)}
                      className="w-full px-2 py-1 rounded bg-[var(--bg-elevated)] border border-[var(--accent)] text-sm text-[var(--text)] outline-none"
                      autoFocus
                    />
                  ) : (
                    <div className="flex items-center justify-between gap-2">
                      <span
                        className={`text-sm font-bold truncate ${
                          isActive ? 'text-[var(--accent)]' : 'text-[var(--text)]'
                        }`}
                      >
                        {scene.name}
                      </span>
                      <button
                        onClick={(e) => {
                          e.stopPropagation();
                          setRenamingIdx(scene.index);
                          setRenameValue(scene.name);
                        }}
                        className="text-[10px] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors shrink-0 cursor-pointer"
                      >
                        Edit
                      </button>
                    </div>
                  )}

                  <div className="mt-2 text-[10px] text-[var(--text-muted)]">
                    {scene.slot_count} slots
                    {isActive && (
                      <span className="ml-2 text-[var(--accent)] font-medium">Active</span>
                    )}
                  </div>

                  {/* Actions row */}
                  <div className="mt-3 flex items-center gap-1.5">
                    <button
                      onClick={() => handleSelectScene(scene.index)}
                      className={`flex-1 px-2 py-1 rounded text-xs font-medium transition-colors cursor-pointer ${
                        isActive
                          ? 'bg-[var(--accent)] text-white'
                          : 'bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-[var(--text)]'
                      }`}
                    >
                      {isActive ? 'Active' : 'Load'}
                    </button>
                    <button
                      onClick={() => handleOverwriteScene(scene.index)}
                      title="Overwrite with current chain"
                      className="px-2 py-1 rounded text-xs bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
                    >
                      Overwrite
                    </button>
                    <button
                      onClick={(e) => {
                        e.stopPropagation();
                        if (isConfirming) {
                          handleDeleteScene(scene.index);
                        } else {
                          setConfirmDelete(scene.index);
                        }
                      }}
                      className={`px-2 py-1 rounded text-xs transition-colors cursor-pointer ${
                        isConfirming
                          ? 'bg-red-800 text-red-200'
                          : 'bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-red-400'
                      }`}
                    >
                      {isConfirming ? 'Confirm?' : 'Delete'}
                    </button>
                  </div>

                  {/* Reorder buttons */}
                  <div className="mt-2 flex items-center gap-1">
                    <button
                      onClick={() => {
                        if (scene.index > 0) handleReorder(scene.index, scene.index - 1);
                      }}
                      disabled={scene.index === 0}
                      className="flex-1 px-2 py-1 rounded text-[10px] bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-[var(--text)] disabled:opacity-30 disabled:cursor-not-allowed transition-colors cursor-pointer"
                    >
                      Move Up
                    </button>
                    <button
                      onClick={() => {
                        if (scene.index < scenes.length - 1) handleReorder(scene.index, scene.index + 1);
                      }}
                      disabled={scene.index === scenes.length - 1}
                      className="flex-1 px-2 py-1 rounded text-[10px] bg-[var(--bg-elevated)] text-[var(--text-muted)] hover:text-[var(--text)] disabled:opacity-30 disabled:cursor-not-allowed transition-colors cursor-pointer"
                    >
                      Move Down
                    </button>
                  </div>
                </div>
              );
            })}
          </div>

          {/* Active scene large view — full parameter readout */}
          {activeScene !== null && scenes[activeScene] && (() => {
            const active = scenes.find((s) => s.index === activeScene)!;
            return (
              <SceneDetailView
                sceneName={active.name}
                sceneIndex={active.index}
                slotCount={active.slot_count}
              />
            );
          })()}

          {/* Navigation controls */}
          <div className="flex items-center gap-3">
            <button
              onClick={handlePrevScene}
              className="px-6 py-3 rounded-xl bg-zinc-700 text-white font-medium text-sm hover:bg-zinc-600 transition-colors cursor-pointer"
            >
              Prev Scene
            </button>
            <button
              onClick={handleNextScene}
              className="px-6 py-3 rounded-xl bg-green-700 text-white font-medium text-sm hover:bg-green-600 transition-colors cursor-pointer"
            >
              Next Scene
            </button>
            <div className="flex-1" />
            <span className="text-xs text-[var(--text-muted)]">
              {activeScene !== null
                ? `Scene ${activeScene + 1} of ${scenes.length}`
                : 'No active scene'}
            </span>
          </div>
        </>
      )}
    </div>
  );
}
