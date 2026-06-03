import { useState } from 'react';
import { useEngineStore } from '../stores/engineStore';
import { useSettingsStore } from '../stores/settingsStore';

interface SetupWizardProps {
  onClose: () => void;
}

const steps = [
  {
    id: 'connect',
    title: 'Connect Your Guitar',
    description: 'Plug your guitar into an audio interface or directly into your computer\'s audio input.',
    icon: (
      <svg className="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M12 2a4 4 0 014 4v2h-8V6a4 4 0 014-4z" />
        <path d="M8 8h8v4a4 4 0 01-8 0V8z" />
        <path d="M12 16v6" />
        <path d="M9 22h6" />
      </svg>
    ),
  },
  {
    id: 'select-input',
    title: 'Select Input Device',
    description: 'Go to Settings → Audio Backend and choose the device your guitar is plugged into.',
    icon: (
      <svg className="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <rect x="4" y="4" width="16" height="16" rx="2" />
        <path d="M9 9h6v6H9z" />
        <path d="M9 1v3M15 1v3M9 20v3M15 20v3M20 9h3M20 15h3M1 9h3M1 15h3" />
      </svg>
    ),
  },
  {
    id: 'engine',
    title: 'Choose Engine Mode',
    description: 'Internal uses Kicks\' built-in amp sim. Guitarix mode requires guitarix installed. Auto tries both.',
    icon: (
      <svg className="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <path d="M12 2l3 6h6l-5 4 2 6-6-4-6 4 2-6-5-4h6z" />
      </svg>
    ),
  },
  {
    id: 'start',
    title: 'Start the Engine',
    description: 'Hit the START button or press Space. Watch the VU meters — if they move, your guitar is connected.',
    icon: (
      <svg className="w-8 h-8" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5">
        <circle cx="12" cy="12" r="10" />
        <path d="M10 8l6 4-6 4V8z" />
      </svg>
    ),
  },
];

export function SetupWizard({ onClose }: SetupWizardProps) {
  const [step, setStep] = useState(0);
  const [dontShow, setDontShow] = useState(false);
  const status = useEngineStore((s) => s.status);
  const settings = useSettingsStore((s) => s.settings);

  const current = steps[step];
  const isLast = step === steps.length - 1;

  const handleNext = () => {
    if (isLast) {
      if (dontShow) {
        localStorage.setItem('kicks:wizard-dismissed', 'true');
      }
      onClose();
    } else {
      setStep(step + 1);
    }
  };

  const handleBack = () => {
    if (step > 0) setStep(step - 1);
  };

  // Check if input device is selected
  const hasInputDevice = settings?.input_device && settings.input_device !== '';

  // Check if engine is running
  const engineRunning = status.running;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 backdrop-blur-sm">
      <div className="w-full max-w-lg mx-4 border border-[var(--border)] rounded-2xl bg-[var(--bg-surface)] shadow-2xl overflow-hidden">
        {/* Header */}
        <div className="px-6 py-5 border-b border-[var(--border)]">
          <div className="flex items-center justify-between">
            <h2 className="text-lg font-bold text-[var(--text-h)]">Welcome to Kicks</h2>
            <button
              onClick={onClose}
              className="text-[var(--text-muted)] hover:text-[var(--text)] transition-colors cursor-pointer"
            >
              <svg className="w-5 h-5" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2">
                <path d="M18 6L6 18M6 6l12 12" />
              </svg>
            </button>
          </div>
          <p className="text-sm text-[var(--text-muted)] mt-1">
            Quick setup to get your guitar sounding great.
          </p>
        </div>

        {/* Progress */}
        <div className="flex items-center gap-1 px-6 pt-4">
          {steps.map((s, i) => (
            <div key={s.id} className="flex-1 flex items-center gap-1">
              <div
                className={`h-1.5 rounded-full flex-1 transition-colors ${
                  i <= step ? 'bg-[var(--accent)]' : 'bg-[var(--border)]'
                }`}
              />
            </div>
          ))}
        </div>

        {/* Step content */}
        <div className="px-6 py-6">
          <div className="flex items-start gap-4">
            <div className={`shrink-0 p-3 rounded-xl ${
              step === 0 ? 'bg-[var(--accent)]/10 text-[var(--accent)]' :
              step === 1 ? 'bg-[var(--success)]/10 text-[var(--success)]' :
              step === 2 ? 'bg-[var(--warning)]/10 text-[var(--warning)]' :
              'bg-[var(--danger)]/10 text-[var(--danger)]'
            }`}>
              {current.icon}
            </div>
            <div className="flex-1">
              <h3 className="text-base font-semibold text-[var(--text)]">
                {step + 1}. {current.title}
              </h3>
              <p className="text-sm text-[var(--text-muted)] mt-1 leading-relaxed">
                {current.description}
              </p>

              {/* Contextual hints */}
              {current.id === 'select-input' && hasInputDevice && (
                <div className="mt-3 px-3 py-2 rounded-lg bg-[var(--success)]/10 border border-[var(--success)]/20 text-sm text-[var(--success)]">
                  Input device selected: <strong>{settings?.input_device}</strong>
                </div>
              )}
              {current.id === 'select-input' && !hasInputDevice && (
                <div className="mt-3 px-3 py-2 rounded-lg bg-[var(--warning)]/10 border border-[var(--warning)]/20 text-sm text-[var(--warning)]">
                  No input device selected yet. Go to Settings → Audio Backend.
                </div>
              )}
              {current.id === 'start' && engineRunning && (
                <div className="mt-3 px-3 py-2 rounded-lg bg-[var(--success)]/10 border border-[var(--success)]/20 text-sm text-[var(--success)]">
                  Engine is running! Strum your guitar and watch the VU meters.
                </div>
              )}
            </div>
          </div>
        </div>

        {/* Footer */}
        <div className="px-6 py-4 border-t border-[var(--border)] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <label className="flex items-center gap-2 text-sm text-[var(--text-muted)] cursor-pointer">
              <input
                type="checkbox"
                checked={dontShow}
                onChange={(e) => setDontShow(e.target.checked)}
                className="rounded border-[var(--border)] bg-[var(--bg)] text-[var(--accent)]"
              />
              Don't show again
            </label>
          </div>
          <div className="flex items-center gap-2">
            {step > 0 && (
              <button
                onClick={handleBack}
                className="px-4 py-2 rounded-lg border border-[var(--border)] text-sm text-[var(--text)] hover:bg-[var(--bg-elevated)] transition-colors cursor-pointer"
              >
                Back
              </button>
            )}
            <button
              onClick={handleNext}
              className="px-4 py-2 rounded-lg bg-[var(--accent)] text-[#07070a] text-sm font-bold hover:opacity-90 transition-opacity cursor-pointer neon-button"
            >
              {isLast ? 'Get Started' : 'Next'}
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
