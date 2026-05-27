import { useEffect, useState, useCallback } from 'react';
import * as api from '../lib/tauri';
import type { NamModelInfo } from '../types/tauri';

const ARCHITECTURE_COLORS: Record<string, string> = {
  'WaveNet': 'bg-cyan-700',
  'LSTM': 'bg-violet-700',
  'GRU': 'bg-pink-700',
  'Transformer': 'bg-amber-700',
  'CNN': 'bg-emerald-700',
  'RTNeural': 'bg-sky-700',
};

export function NAMBrowser() {
  const [files, setFiles] = useState<{ path: string; name: string }[]>([]);
  const [loading, setLoading] = useState(true);
  const [selected, setSelected] = useState<string | null>(null);
  const [loadedModel, setLoadedModel] = useState<NamModelInfo | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const results = await api.listNamFiles();
        setFiles(results);
      } catch (err) {
        setError('Failed to list NAM files');
        console.error('Failed to list NAM files:', err);
      } finally {
        setLoading(false);
      }
    })();
    (async () => {
      try {
        const info = await api.getNamInfo();
        setLoadedModel(info);
      } catch (err) {
        console.error('Failed to get loaded NAM info:', err);
      }
    })();
  }, []);

  const handleLoadModel = useCallback(async () => {
    if (!selected) return;
    setLoading(true);
    setError(null);
    try {
      const result = await api.loadNamModel(selected);
      setLoadedModel(result);
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to load NAM model');
    } finally {
      setLoading(false);
    }
  }, [selected]);

  const handleClearModel = useCallback(async () => {
    setError(null);
    try {
      await api.clearNamModel();
      setLoadedModel(null);
    } catch (err) {
      setError('Failed to clear NAM model');
    }
  }, []);

  return (
    <div className="flex flex-col gap-5 max-w-[1000px]">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="font-display text-xl font-bold text-[var(--text-h)] tracking-wider">NAM MODELS</h2>
          <p className="text-[13px] text-[var(--text-muted)] mt-1">
            Neural Amp Modeler — load captured amp models for AI-powered tone.
          </p>
        </div>
      </div>

      {/* Active model card */}
      {loadedModel && (
        <div className="border border-[var(--accent)]/30 bg-[var(--accent-bg)] rounded-xl p-4 flex items-center gap-4">
          <div className={`px-2.5 py-1 rounded text-[10px] font-bold text-white ${ARCHITECTURE_COLORS[loadedModel.architecture] ?? 'bg-zinc-600'}`}>
            {loadedModel.architecture}
          </div>
          <div className="flex-1 min-w-0">
            <div className="text-sm font-bold text-[var(--text)] truncate">{loadedModel.file_name}</div>
            <div className="flex items-center gap-3 mt-1">
              <span className="text-[10px] text-[var(--text-muted)] font-mono-data">{loadedModel.sample_rate / 1000}kHz</span>
              <span className="text-[10px] text-[var(--text-muted)] font-mono-data">{loadedModel.num_parameters.toLocaleString()} params</span>
              <span className="text-[10px] text-[var(--accent)] font-medium">ACTIVE</span>
            </div>
          </div>
          <button
            onClick={handleClearModel}
            className="px-3 py-1.5 rounded-lg border border-[var(--border)] text-xs text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
          >
            Unload
          </button>
        </div>
      )}

      {/* Error */}
      {error && (
        <div className="border border-[var(--danger)]/30 bg-[var(--danger-bg)] rounded-xl p-3 text-sm text-[var(--danger)]">
          {error}
        </div>
      )}

      {/* File list */}
      {loading ? (
        <div className="border border-[var(--border)] rounded-xl p-12 flex items-center justify-center bg-[var(--bg-surface)]">
          <span className="text-[var(--text-muted)] text-sm animate-pulse">Scanning NAM directories...</span>
        </div>
      ) : files.length === 0 ? (
        <div className="border-2 border-dashed border-[var(--border)] rounded-xl p-12 flex flex-col items-center justify-center gap-3 bg-[var(--bg-surface)]">
          <span className="text-3xl text-[var(--text-muted)] opacity-30">🧠</span>
          <span className="text-[var(--text-muted)] text-sm text-center">
            No NAM models found. Configure directories in Settings.
          </span>
          <span className="text-[var(--text-muted)] text-[10px]">
            Supports .nam, .json, .onnx model files
          </span>
        </div>
      ) : (
        <div className="border border-[var(--border)] rounded-xl overflow-hidden">
          {files.map((file) => {
            const isSelected = selected === file.path;
            const isActive = loadedModel?.path === file.path;
            return (
              <div
                key={file.path}
                onClick={() => setSelected(isSelected ? null : file.path)}
                className={`flex items-center gap-4 px-4 py-3 border-b border-[var(--border)] last:border-none cursor-pointer transition-colors ${
                  isActive
                    ? 'bg-[var(--accent-bg)] ring-1 ring-inset ring-[var(--accent)]/20'
                    : isSelected
                      ? 'bg-[var(--bg-elevated)]'
                      : 'hover:bg-[var(--surface-hover)]'
                }`}
              >
                <span className="text-lg opacity-60">🧠</span>
                <div className="flex-1 min-w-0">
                  <div className="text-sm text-[var(--text)] truncate font-medium">
                    {file.name}
                    {isActive && <span className="ml-2 text-[10px] text-[var(--accent)]">(active)</span>}
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] truncate font-mono-data">{file.path}</div>
                </div>
                {isSelected && (
                  <button
                    onClick={(e) => { e.stopPropagation(); handleLoadModel(); }}
                    className="px-3 py-1.5 rounded-md bg-[var(--accent)] text-[#07070a] text-xs font-bold hover:opacity-90 transition-opacity cursor-pointer"
                  >
                    Load
                  </button>
                )}
              </div>
            );
          })}
        </div>
      )}

      {/* Info box */}
      <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
        <h4 className="font-display text-[10px] text-[var(--text-muted)] uppercase tracking-[0.15em] mb-2">About NAM</h4>
        <p className="text-[11px] text-[var(--text-muted)] leading-relaxed">
          Neural Amp Modeler uses deep learning to capture the exact sound of real guitar amps, pedals, and cabinets.
          Load a .nam model file to replace the Amp and Cab plugins with a neural network that precisely emulates the captured gear.
          Models are trained from real audio recordings and run in real-time using ONNX inference.
        </p>
      </div>
    </div>
  );
}
