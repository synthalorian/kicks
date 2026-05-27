import { useEffect, useState, useCallback } from 'react';
import * as api from '../lib/tauri';
import type { IrFileInfo, IrLoadResult } from '../types/tauri';
import { useHotkeys } from '../hooks/useHotkeys';

export function IRBrowser() {
  const [files, setFiles] = useState<IrFileInfo[]>([]);
  const [loading, setLoading] = useState(true);
  const [loadingIr, setLoadingIr] = useState(false);
  const [selected, setSelected] = useState<string | null>(null);
  const [picking, setPicking] = useState(false);
  const [loadedIr, setLoadedIr] = useState<IrLoadResult | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    (async () => {
      try {
        const results = await api.listIrFiles();
        setFiles(results);
      } catch (err) {
        setError('Failed to list IR files');
        console.error('Failed to list IR files:', err);
      } finally {
        setLoading(false);
      }
    })();
    (async () => {
      try {
        const info = await api.getCabIrInfo();
        setLoadedIr(info);
      } catch (err) {
        console.error('Failed to get loaded IR info:', err);
      }
    })();
  }, []);

  const loadFiles = async () => {
    setLoading(true);
    setError(null);
    try {
      const results = await api.listIrFiles();
      setFiles(results);
    } catch (err) {
      setError('Failed to list IR files');
      console.error('Failed to list IR files:', err);
    } finally {
      setLoading(false);
    }
  };

  const handlePickFile = useCallback(async () => {
    setPicking(true);
    setError(null);
    try {
      const result = await api.pickIrFile();
      if (result) {
        setFiles((prev) => {
          const exists = prev.some((f) => f.path === result.path);
          if (!exists) return [result, ...prev];
          return prev;
        });
        setSelected(result.path);
      }
    } catch (err) {
      console.error('Failed to pick IR file:', err);
    } finally {
      setPicking(false);
    }
  }, []);

  const handleDrop = useCallback(async (e: React.DragEvent) => {
    e.preventDefault();
    console.log('Drop zone activated');
  }, []);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
  }, []);

  const handleLoadToCab = useCallback(async () => {
    if (!selected) return;
    setLoadingIr(true);
    setError(null);
    try {
      const result = await api.loadIrToCab(selected);
      setLoadedIr(result);
      console.log('IR loaded:', result.file_name);
    } catch (err) {
      setError(typeof err === 'string' ? err : 'Failed to load IR');
      console.error('Failed to load IR:', err);
    } finally {
      setLoadingIr(false);
    }
  }, [selected]);

  const handleClearIr = useCallback(async () => {
    setError(null);
    try {
      await api.clearCabIr();
      setLoadedIr(null);
      console.log('IR cleared');
    } catch (err) {
      setError('Failed to clear IR');
      console.error('Failed to clear IR:', err);
    }
  }, []);

  const activeDir = files.length > 0
    ? files[0].path.substring(0, files[0].path.lastIndexOf('/'))
    : 'No directory configured';

  useHotkeys([
    { key: 'o', ctrl: true, handler: handlePickFile, ignoreInput: true },
  ]);

  return (
    <div className="flex flex-col gap-6">
      {/* Header */}
      <div className="flex items-center justify-between">
        <div>
          <h2 className="text-2xl font-bold text-[var(--text-h)]">IR Browser</h2>
          <p className="text-sm text-[var(--text-muted)] mt-1">
            Browse and load impulse responses for cabinet simulation.
          </p>
        </div>
        <button
          onClick={handlePickFile}
          disabled={picking}
          className="px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity disabled:opacity-40 cursor-pointer"
        >
          {picking ? 'Selecting...' : 'Browse Files'}
        </button>
      </div>

      {/* Error display */}
      {error && (
        <div className="border border-red-500/30 bg-red-500/10 rounded-xl p-3 text-sm text-red-400">
          {error}
        </div>
      )}

      {/* Currently loaded IR status */}
      {loadedIr && (
        <div className="border border-[var(--accent)]/30 bg-[var(--accent-bg)] rounded-xl p-4 flex items-center gap-3">
          <div className="flex-1 min-w-0">
            <div className="flex items-center gap-2">
              <span className="text-sm font-medium text-[var(--text)]">
                Active IR: {loadedIr.file_name}
              </span>
            </div>
            <div className="flex items-center gap-3 mt-1">
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                {loadedIr.sample_rate / 1000}kHz
              </span>
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                {loadedIr.length_samples} samples
              </span>
              <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                {loadedIr.length_ms}ms
              </span>
              <span className="text-xs text-[var(--accent)] font-medium">Active</span>
            </div>
          </div>
          <button
            onClick={handleClearIr}
            className="px-3 py-1.5 rounded-lg border border-[var(--border)] text-sm text-[var(--text-muted)] hover:text-[var(--text)] hover:border-red-500/50 transition-colors cursor-pointer"
          >
            Clear IR
          </button>
        </div>
      )}

      {/* Directory info */}
      <div className="text-xs text-[var(--text-muted)] flex items-center gap-2">
        <span className="font-mono bg-[var(--bg-surface)] px-2 py-1 rounded border border-[var(--border)] truncate max-w-md">
          {activeDir}
        </span>
        <span className="text-[var(--text-muted)]">
          {files.length} file{files.length !== 1 ? 's' : ''}
        </span>
        <button
          onClick={loadFiles}
          className="ml-auto px-2 py-1 rounded text-xs border border-[var(--border)] text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
        >
          Refresh
        </button>
      </div>

      {/* Loading state */}
      {loading && (
        <div className="border border-[var(--border)] rounded-xl p-12 flex items-center justify-center bg-[var(--bg-surface)]">
          <span className="text-[var(--text-muted)] text-sm">Scanning IR directories...</span>
        </div>
      )}

      {/* Drop zone when empty */}
      {!loading && files.length === 0 && (
        <div
          onDrop={handleDrop}
          onDragOver={handleDragOver}
          className="border-2 border-dashed border-[var(--border)] rounded-xl p-12 flex flex-col items-center justify-center gap-3 bg-[var(--bg-surface)] hover:border-[var(--accent)] transition-colors cursor-pointer"
        >
          <span className="text-3xl text-[var(--text-muted)] opacity-50">IR</span>
          <span className="text-[var(--text-muted)] text-sm">
            No IR files found. Configure directories in Settings, or click Browse to pick a file.
          </span>
          <span className="text-[var(--text-muted)] text-[10px]">
            Supports .wav, .nam, .irs
          </span>
        </div>
      )}

      {/* File list */}
      {!loading && files.length > 0 && (
        <div className="border border-[var(--border)] rounded-xl overflow-hidden">
          {files.map((file) => {
            const isSelected = selected === file.path;
            const isWav = file.sample_rate > 0;
            const isActive = loadedIr?.path === file.path;
            return (
              <div
                key={file.path}
                onClick={() => setSelected(isSelected ? null : file.path)}
                className={`flex items-center gap-4 px-4 py-3 border-b border-[var(--border)] last:border-none cursor-pointer transition-colors ${
                  isActive
                    ? 'bg-[var(--accent-bg)] ring-1 ring-inset ring-[var(--accent)]/30'
                    : isSelected
                      ? 'bg-[var(--bg-surface)]'
                      : 'hover:bg-[var(--surface-hover)]'
                }`}
              >
                <span className="text-lg">{isWav ? 'IR' : 'FI'}</span>

                {/* Name & path */}
                <div className="flex-1 min-w-0">
                  <div className="text-sm text-[var(--text)] truncate font-medium">
                    {file.name}
                    {isActive && (
                      <span className="ml-2 text-[10px] text-[var(--accent)]">(active)</span>
                    )}
                  </div>
                  <div className="text-[10px] text-[var(--text-muted)] truncate">
                    {file.path}
                  </div>
                </div>

                {/* Metadata chips */}
                <div className="flex items-center gap-2 shrink-0">
                  {isWav && (
                    <>
                      <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                        {file.sample_rate / 1000}kHz
                      </span>
                      <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                        {file.duration_ms}ms
                      </span>
                      <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                        {file.channels}ch
                      </span>
                    </>
                  )}
                  {!isWav && (
                    <span className="text-[10px] px-1.5 py-0.5 rounded bg-[var(--bg-elevated)] text-[var(--text-muted)]">
                      Unparsed
                    </span>
                  )}
                  {isSelected && (
                    <span className="text-xs text-[var(--accent)] font-medium">Selected</span>
                  )}
                </div>
              </div>
            );
          })}
        </div>
      )}

      {/* Load to Cab */}
      {selected && (
        <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)] flex items-center gap-3">
          <div className="flex-1 min-w-0">
            <div className="text-sm text-[var(--text)] truncate font-medium">
              {files.find((f) => f.path === selected)?.name ?? selected}
            </div>
            <div className="text-[10px] text-[var(--text-muted)] truncate">{selected}</div>
          </div>
          <button
            onClick={handleLoadToCab}
            disabled={loadingIr}
            className="px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity disabled:opacity-40 cursor-pointer"
          >
            {loadingIr ? 'Loading...' : 'Load to Cab'}
          </button>
        </div>
      )}
    </div>
  );
}
