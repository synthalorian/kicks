import { useState, useCallback } from 'react';
import * as api from '../lib/tauri';
import { usePresetsStore } from '../stores/presetsStore';
import type { AiPresetResult } from '../types/tauri';
import { useHotkeys } from '../hooks/useHotkeys';

const TONE_EXAMPLES = [
  'Creamy blues lead with warm reverb',
  'Heavy metal chug with tight gate',
  'Sparkly clean with chorus and delay',
  'Punky crunch with slapback echo',
  'Smooth jazz with gentle compression',
];

export function AIAssistant() {
  const [prompt, setPrompt] = useState('');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [result, setResult] = useState<AiPresetResult | null>(null);
  const [applying, setApplying] = useState(false);
  const [saving, setSaving] = useState(false);
  const savePreset = usePresetsStore((s) => s.savePreset);

  const handleGenerate = useCallback(async () => {
    if (!prompt.trim()) return;
    setLoading(true);
    setError(null);
    setResult(null);

    try {
      const aiResult = await api.generateAiPreset(prompt.trim());
      setResult(aiResult);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setLoading(false);
    }
  }, [prompt]);

  const handleApply = useCallback(async () => {
    if (!result) return;
    setApplying(true);
    try {
      await api.applyAiPreset(result.signal_chain);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setApplying(false);
    }
  }, [result]);

  const handleSavePreset = useCallback(async () => {
    if (!result) return;
    setSaving(true);
    try {
      // Apply first to set the signal chain, then save
      await api.applyAiPreset(result.signal_chain);
      await savePreset('AI Generated', result.name, result.description, ['ai']);
    } catch (err) {
      setError(err instanceof Error ? err.message : String(err));
    } finally {
      setSaving(false);
    }
  }, [result, savePreset]);

  useHotkeys([
    { key: 'Enter', ctrl: true, handler: handleGenerate, ignoreInput: false },
  ]);

  return (
    <div className="flex flex-col gap-6 max-w-2xl">
      <div>
        <h2 className="text-2xl font-bold text-[var(--text-h)]">AI Tone Assistant</h2>
        <p className="text-sm text-[var(--text-muted)] mt-1">
          Describe the sound you want, and the AI will generate a preset.
        </p>
      </div>

      {/* Input */}
      <div className="flex gap-3">
        <input
          type="text"
          value={prompt}
          onChange={(e) => setPrompt(e.target.value)}
          placeholder='e.g., "Creamy blues lead with warm reverb"'
          className="flex-1 px-4 py-3 rounded-xl border border-[var(--border)] bg-[var(--bg-surface)] text-[var(--text)] text-sm outline-none focus:border-[var(--accent)] transition-colors"
          onKeyDown={(e) => e.key === 'Enter' && !loading && handleGenerate()}
        />
        <button
          onClick={handleGenerate}
          disabled={loading || !prompt.trim()}
          className="px-6 py-3 rounded-xl bg-[var(--accent)] text-white font-medium text-sm hover:opacity-90 transition-opacity disabled:opacity-40 cursor-pointer"
        >
          {loading ? 'Thinking...' : 'Generate'}
        </button>
      </div>

      {/* Examples */}
      <div className="flex flex-wrap gap-2">
        {TONE_EXAMPLES.map((ex) => (
          <button
            key={ex}
            onClick={() => setPrompt(ex)}
            className="px-3 py-1.5 rounded-full border border-[var(--border)] text-xs text-[var(--text-muted)] hover:text-[var(--text)] hover:border-zinc-600 transition-colors cursor-pointer"
          >
            {ex}
          </button>
        ))}
      </div>

      {/* Loading */}
      {loading && (
        <div className="border border-[var(--border)] rounded-xl p-6 bg-[var(--bg-surface)]">
          <div className="flex items-center gap-3 text-sm text-[var(--text-muted)]">
            <span className="animate-pulse">Analyzing tone description...</span>
          </div>
        </div>
      )}

      {/* Error */}
      {error && (
        <div className="border border-red-700 rounded-xl p-4 bg-red-900/20">
          <p className="text-sm text-red-400">{error}</p>
        </div>
      )}

      {/* Result */}
      {result && (
        <div className="border border-[var(--border)] rounded-xl p-6 bg-[var(--bg-surface)] space-y-4">
          <div>
            <h3 className="text-base font-bold text-[var(--text)]">{result.name}</h3>
            {result.description && (
              <p className="text-sm text-[var(--text-muted)] mt-1">{result.description}</p>
            )}
          </div>

          <div className="space-y-2">
            {result.signal_chain.slots.map((slot, i) => {
              const paramEntries = Object.entries(slot.parameters);
              return (
                <div
                  key={i}
                  className="border border-[var(--border)] rounded-lg p-3 bg-[var(--bg)]"
                >
                  <div className="flex items-center justify-between mb-2">
                    <span className="text-sm font-medium text-[var(--text)]">
                      {slot.plugin_type}
                    </span>
                    {!slot.enabled && (
                      <span className="text-xs text-[var(--text-muted)]">(disabled)</span>
                    )}
                  </div>
                  {paramEntries.length > 0 && (
                    <div className="flex flex-wrap gap-x-4 gap-y-1">
                      {paramEntries.map(([key, val]) => (
                        <span key={key} className="text-xs text-[var(--text-muted)]">
                          {key}: <span className="text-[var(--text)]">{val.toFixed(2)}</span>
                        </span>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>

          <div className="flex gap-3">
            <button
              onClick={handleApply}
              disabled={applying}
              className="px-4 py-2 rounded-lg bg-[var(--accent)] text-white text-sm font-medium hover:opacity-90 transition-opacity disabled:opacity-40 cursor-pointer"
            >
              {applying ? 'Applying...' : 'Apply to Signal Chain'}
            </button>
            <button
              onClick={handleSavePreset}
              disabled={saving}
              className="px-4 py-2 rounded-lg border border-[var(--border)] text-[var(--text-muted)] text-sm hover:text-[var(--text)] transition-colors disabled:opacity-40 cursor-pointer"
            >
              {saving ? 'Saving...' : 'Save as Preset'}
            </button>
          </div>
        </div>
      )}

      {/* Info */}
      <div className="border border-[var(--border)] rounded-xl p-4 bg-[var(--bg-surface)]">
        <h4 className="text-xs font-medium text-[var(--text-muted)] uppercase tracking-wider mb-2">
          How it works
        </h4>
        <div className="text-xs text-[var(--text-muted)] leading-relaxed space-y-1">
          <p>
            Configure your AI provider (Anthropic or OpenAI-compatible) in Settings first.
            The AI model generates structured JSON that maps directly to the signal chain.
          </p>
        </div>
      </div>
    </div>
  );
}
