# Signal Chain

The Signal Chain page is where you build and arrange your effect chain. It has
two sections: the **PedalBoard** (where you arrange pedals) and the **AudioFlow**
(real-time level visualization).

## Engine Controls

At the top of the page you'll see:

- **Engine status badge** — green "Engine Running" or red "Engine Stopped"
- **Start Engine** / **Stop Engine** button
- Press **Space** to toggle the engine on/off from anywhere

## PedalBoard

The PedalBoard shows each plugin as a card (pedal) in the chain.

### Adding plugins
- Click **+ Add Slot** to append a new pedal to the chain
- Choose the plugin type from the dropdown

### Reordering
- **Drag and drop** pedals to reorder them in the chain
- The audio processing order follows the visual order

### Enabling / Disabling
- Each pedal has a toggle switch — disabled pedals are bypassed (shown with dashed border at reduced opacity)

### Wet/Dry Mix
- Each pedal has a wet/dry mix slider
- 100% = fully wet (default), 0% = fully dry (bypassed)

### Parameter Knobs
- Click a pedal to expand its parameter controls
- Adjust sliders for parameters like gain, drive, bass, mid, treble, master (Amp plugin)
- Changes are applied **lock-free** — the audio callback never blocks on parameter updates

## Amp vs BassAmp

The Amp plugin has a **BassAmp mode** toggle:

| Mode | Low Shelf | Mid Peaking | High Shelf | Best for |
|------|-----------|-------------|------------|----------|
| Guitar (default) | 250 Hz | 800 Hz | 3 kHz | Standard guitar |
| Bass | 100 Hz | 500 Hz | 4 kHz | Bass guitar |

Enable BassAmp mode in the Amp pedal's parameters (`bass_mode: 1.0`), or select
a BassAmp slot from the add-menu.

## AudioFlow Visualization

Below the PedalBoard, the AudioFlow shows:

- **Per-plugin VU meters** — real-time RMS levels updated every ~50ms
- **Color zones:**
  - 🟢 Green — safe levels (&lt; 25%)
  - 🟡 Amber — warm (25-80%)
  - 🟠 Orange — hot (80-92%)
  - 🔴 Red — clipping (&gt; 92%)
- **dBFS labels** — each meter shows the level in dB below full scale
- **0 dBFS tickmark** — a red vertical line at 0 dBFS
- **Output level** — the last plugin's level shown in the header
- **Legend** — Safe / Warm / Hot indicator with 0 dBFS reference

## Default Chain

When the engine starts with no saved chain, it creates:

```
Boost → Amp → Cab → Delay → Reverb
```

## Undo / Redo

- **Ctrl+Z** — undo the last signal chain change (add, remove, reorder, parameter changes)
- **Ctrl+Shift+Z** — redo
- Up to **50 undo steps** are preserved
