interface ParamSliderProps {
  label: string;
  value: number;
  min?: number;
  max?: number;
  step?: number;
  onChange: (value: number) => void;
}

export function ParamSlider({ label, value, min = 0, max = 1, step = 0.01, onChange }: ParamSliderProps) {
  const pct = ((value - min) / (max - min)) * 100;
  const displayVal = step >= 1 ? value.toFixed(0) : value.toFixed(2);

  return (
    <div className="flex flex-col gap-1">
      <div className="flex items-center justify-between text-xs">
        <span className="text-[var(--text-muted)]">{label}</span>
        <span className="text-[var(--text)] font-mono">{displayVal}</span>
      </div>
      <div className="relative h-5 flex items-center">
        <input
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(e) => onChange(parseFloat(e.target.value))}
          className="absolute inset-0 w-full h-full opacity-0 cursor-pointer z-10"
        />
        {/* Custom track */}
        <div className="w-full h-1.5 rounded-full bg-[var(--bg)] overflow-hidden">
          <div
            className="h-full rounded-full bg-[var(--accent)] transition-[width]"
            style={{ width: `${pct}%` }}
          />
        </div>
      </div>
    </div>
  );
}
