# Kicks Guitar Workstation 🦎🎸

**Kicks** is an open-source guitar & bass workstation for Linux — a desktop app
for guitar and bass tone shaping with a native DSP engine. Built with Rust +
Tauri 2 + React. All state persists to `~/.config/kicks/`.

## Current Status

```
▲ Data persistence            —  ✅ Presets, config, signal chain, scenes, MIDI → ~/.config/kicks/
▲ Signal chain reorder        —  ✅ move_slot command + drag-drop fully wired
▲ IR file loading             —  ✅ File dialog, WAV metadata parsing, directory scanning
▲ Frontend UI (7 pages)       —  ✅ React 19 + Tailwind CSS 4 + error boundaries
▲ Tauri IPC layer             —  ✅ 30+ commands (engine, signal chain, presets, IR, settings, MIDI, AI, amp presets, scenes)
▲ Frontend State              —  ✅ Zustand stores + Tauri invoke wrapper + simulated dev fallback
▲ Native DSP engine           —  ✅ Boost → Amp/BassAmp (3-band EQ + waveshaper) → Cab (LP/HP + convolution IR) → Delay → Reverb
▲ Lock-free param channel     —  ✅ SPSC ring buffer (set_parameter never blocks audio callback)
▲ Real-time audio I/O         —  ✅ CPAL full-duplex (ALSA/PipeWire/CoreAudio/WASAPI), JACK feature-gated
▲ Audio flow visualization    —  ✅ Per-plugin RMS VU meters with dBFS labels, polled at 20 Hz
▲ Audio device enumeration    —  ✅ CPAL device listing in Settings dropdown, auto-restart on config change
▲ guitarix-rpc                —  ✅ Full JSON-RPC 2.0 client (presets, MIDI, tuner, banks, plugins, LADSPA)
▲ Data models                 —  ✅ Signal chain, presets, MIDI mapping, config, scenes, amp presets
▲ MIDI device integration     —  ✅ Device discovery, connection, learn mode, CC→parameter mapping via midir
▲ AI Tone Assistant           —  ✅ Multi-provider (Anthropic + OpenAI-compatible: OpenAI, OpenRouter, Ollama, llama-server, GLM, Kimi, etc.)
▲ Convolution IR in Cab       —  ✅ Direct-form convolver (8192 sample max IR), WAV loading, mono downmix
▲ Live Mode / Scenes          —  ✅ Scene CRUD with setlist management, next/prev navigation, persistence
▲ Built-in Amp Presets        —  ✅ 300+ guitar amp presets + 61 bass amp presets across 16 genres
▲ Bass Guitar Support         —  ✅ BassAmp plugin type with shifted EQ (100Hz/500Hz/4kHz), dedicated presets
▲ Undo/Redo                   —  ✅ Signal chain undo/redo (50 steps)
▲ Global Keyboard Shortcuts   —  ✅ Page nav (1-7), engine toggle (space), undo (Ctrl+Z), redo (Ctrl+Shift+Z), save (Ctrl+S)
```

## Architecture

```
kicks/
├── src-tauri/                # Tauri 2 desktop shell (Rust)
│   ├── src/lib.rs            # App entry, AppState, 30 commands registered
│   ├── src/main.rs           # Win subsystem shim
│   ├── src/midi.rs           # MidiManager — midir port discovery + CC event channel
│   ├── src/ai.rs             # Multi-provider AI (Anthropic + OpenAI-compatible)
│   └── src/commands/         # Tauri IPC commands (10 modules)
│       ├── engine.rs         #   Engine lifecycle + parameters
│       ├── signal_chain.rs   #   Slot CRUD + enable/disable + move/reorder + undo/redo
│       ├── presets.rs        #   Bank/preset CRUD with disk persistence
│       ├── settings.rs       #   App configuration with disk persistence
│       ├── ir.rs             #   IR file listing, picker dialog, WAV parsing
│       ├── midi.rs           #   MIDI device discovery, config CRUD, learn mode, event poll
│       ├── ai.rs             #   AI preset generation + apply to signal chain
│       ├── amp_presets.rs    #   Built-in amp/bass preset list + apply
│       └── scenes.rs         #   Scene CRUD, reorder, next/prev navigation
├── crates/
│   ├── guitarix-rpc/         # JSON-RPC 2.0 client → Guitarix engine
│   │   ├── client.rs         #   Full API coverage (banks, presets, MIDI, tuner, etc.)
│   │   ├── connection.rs     #   Exponential backoff + auto-reconnect
│   │   ├── helpers.rs        #   Batch operations (list_params, list_all_presets)
│   │   ├── launcher.rs       #   Headless guitarix -N process manager
│   │   ├── error.rs          #   Typed error enum
│   │   └── types.rs          #   Response structs
│   ├── kicks-core/           # Shared data models + persistence
│   │   ├── signal_chain.rs   #   SignalChain, ChainSlot, PluginType (Amp/BassAmp)
│   │   ├── amp_preset.rs     #   AmpPreset model + 400+ built-in guitar/bass presets
│   │   ├── preset.rs         #   Preset, Bank, PresetCollection
│   │   ├── scene.rs          #   SceneCollection, scene navigation
│   │   ├── midi.rs           #   MidiMapping, MidiConfig
│   │   ├── config.rs         #   KicksConfig, EngineMode, AiProvider
│   │   └── persistence.rs    #   JSON save/load, atomic writes, XDG paths
│   └── kicks-dsp/            # Native DSP engine (no external DSP deps)
│       ├── engine.rs         #   AudioEngine trait + KicksEngine
│       ├── plugins.rs        #   Boost, Amp (guitar/bass), Cab, Delay, Reverb + biquads + convolution IR
│       ├── audio_io.rs       #   CPAL full-duplex I/O (default) + JACK (feature-gated), device enumeration
│       ├── param.rs          #   Lock-free SPSC parameter channel for real-time safety
│       └── convolution.rs    #   Direct-form convolver for IR loading
└── frontend/                 # React 19 + TypeScript 6 + Vite 8 + Tailwind CSS 4
    ├── src/
    │   ├── App.tsx           # Shell with sidebar, toolbar, status bar, error boundary
    │   ├── lib/tauri.ts      # Tauri invoke wrapper (dev fallbacks)
    │   ├── types/tauri.ts    # TypeScript interfaces matching backend types
    │   ├── stores/           # Zustand stores (engine, presets, settings, midi)
    │   ├── components/
    │   │   ├── SignalChain/  # PedalBoard, PedalSlot, ParamSlider
    │   │   ├── AmpPresets/   # AmpPresetSelector with search + category filters
    │   │   ├── ErrorBoundary.tsx
    │   │   ├── Sidebar.tsx
    │   │   ├── Toolbar.tsx
    │   │   └── StatusBar.tsx
    │   └── pages/            # 7 pages — SignalChain, Presets, IRBrowser,
    │                         #   MidiConfig, LiveMode, AIAssistant, Settings
    └── package.json
```

## Data Flow

```
User Input (UI)
     │
     ▼
Tauri IPC Commands (src-tauri/src/commands/)
     │
     ├── Internal Mode ──► kicks-dsp (native DSP) ──► CPAL (ALSA/PipeWire/CoreAudio)
     │                         │
     │                    Lock-free SPSC param channel
     │                         │
     │                    Audio callback → engine.try_lock()
     │
     └── Guitarix Mode ──► guitarix-rpc ──► Guitarix Engine ──► JACK
     
Disk Persistence (~/.config/kicks/):
     ├── config.json            — App settings (engine mode, CPAL, directories, AI key)
     ├── presets.json           — All presets organized by bank
     ├── signal_chain.json      — Auto-saved on every change
     ├── midi_config.json       — MIDI device selection + CC mappings
     └── scenes.json            — Live mode scenes with signal chain snapshots
```


## Requirements

- Rust 1.77+
- Node.js 20+
- ALSA or PipeWire (Linux) — or CoreAudio (macOS), WASAPI (Windows)
- JACK Audio Connection Kit (`jackd`) — optional (for JACK backend feature)
- Guitarix (optional — for companion mode, headless `guitarix -N -p 4040`)

## Build & Run

```bash
# Install system dependencies (Arch Linux)
sudo pacman -S jack2 base-devel libgtk-3 webkit2gtk-4.1

# Install system dependencies (Ubuntu/Debian)
sudo apt install libjack-jackd2-dev libasound2-dev \
  libwebkit2gtk-4.1-dev libgtk-3-dev libayatana-appindicator3-dev

# Install frontend deps
cd frontend && npm install && cd ..

# Development mode — starts both Vite dev server + Tauri window
cargo run

# Run tests
cargo test                          # All 54 tests
cargo test -p kicks-dsp             # DSP tests (26) — plugins, biquads, delay, reverb, convolution, registry, param channel
cargo test -p kicks-core            # Persistence + models (16) — scenes, atomic writes, round-trips
cargo test -p guitarix-rpc          # RPC client tests (12) — connection, backoff, JSON-RPC, launcher

# Production build
cargo build --release
```

### Frontend-only dev (hot-reload in browser)

```bash
cd frontend && npm run dev
```

Opens on `http://localhost:5173` with simulated backend responses.

## Tests

| Crate | Tests | What's covered |
|-------|-------|---------------|
| kicks-dsp | 26 | Plugin processing (boost, amp, cab, delay, reverb), biquad filters, delay line, reverb, convolver (identity, known IR, mixed, empty, trim, normalize, metadata, reset), full chain + parameter mapping, lock-free param channel (send/receive, full-queue error) |
| kicks-core | 16 | Persistence round-trips, atomic writes, JSON load/save, missing file defaults, scene CRUD/navigation/reorder |
| guitarix-rpc | 12 | Connection management, backoff, JSON-RPC call/response, launcher lifecycle |

## Roadmap

### Phase 1 — Core MVP ✅
- [x] Native DSP: Boost, Amp, Cab, Delay, Reverb plugins
- [x] Tauri IPC: engine, signal chain, presets, settings, IR commands
- [x] Frontend stores + state management (zustand)
- [x] Signal Chain page: drag-and-drop pedalboard + reorder
- [x] Presets page: bank browser with CRUD + disk persistence
- [x] MIDI, Live, AI, IR pages with working UI
- [x] Guitarix RPC: complete JSON-RPC 2.0 client
- [x] Disk persistence: presets, config, signal chain, scenes, MIDI

### Phase 2 — Deepen Integration ✅
- [x] MIDI device integration — wire MIDI CC to parameter automation via midir
- [x] AI Tone Assistant — hook up to multi-provider (Anthropic + OpenAI-compatible) API
- [x] Convolution IR loader for Cab plugin (direct-form convolver, WAV loading, mono downmix)
- [x] Live mode scene management — CRUD, persistence, prev/next navigation
- [x] Built-in amp presets — 300+ guitar presets + 61 bass presets
- [x] Bass guitar support — BassAmp plugin type with shifted EQ + dedicated presets
- [x] AI multi-provider refactor — OpenAI-compatible format (OpenAI, OpenRouter, Ollama, llama-server, GLM, Kimi)
- [x] Audio flow visualization in signal chain page

### Phase 3 — Polish & Ship ✅
- [x] Signal chain undo/redo (50 steps)
- [x] Global keyboard shortcuts (page nav, engine toggle, undo/redo, save)
- [x] Tauri bundling config (AppImage, .deb with JACK + PipeWire deps)
- [x] Error boundary wrapper for page-level crash recovery
- [x] CI: Rust + frontend build/test on push/PR
- [x] Real-time audio I/O via PipeWire/ALSA (CPAL)
- [x] Configurable sample rate / buffer size in settings
- [x] Lock-free parameter changes for audio thread safety

### Phase 4 — Ship 🚧
- [ ] Flatpak / Snap packaging
- [ ] User documentation
- [ ] Build-time NAM inference support

## Session Handoff

This section is for continuing work in a new Claude Code session.

### Session 2 (May 24, 2026)

Completed **CPAL audio I/O** — replacing the JACK-only stub with a real cross-platform audio backend.

*Details preserved from Session 1 handoff.*

### Session 3 (May 24, 2026)

Completed **lock-free parameters + audio flow visualization + settings page polish**:

#### Lock-free Parameter Channel
- `crates/kicks-dsp/src/param.rs` — New module: SPSC ring-buffer-based parameter channel (`ParamSender`/`ParamConsumer`)
  - `param_channel()` creates a bounded channel pair (1024 entries)
  - `send()` is lock-free (ringbuf push), safe to call from any thread
  - `ParamSender` implements `Send + Sync` for use from Tauri command handlers
  - `ParamConsumer` is drained in the audio callback before each `process_all`
- `crates/kicks-dsp/src/lib.rs` — Exports `param` module (behind `cpal-backend` feature), `ParamSender`
- `crates/kicks-dsp/src/audio_io.rs` — `CpalAudioIO::start()` accepts `ParamConsumer`, drains it in output callback
- `crates/kicks-dsp/src/plugins.rs` — Added `set_parameter_value()` for main-thread-only HashMap sync while the real plugin update goes through the SPSC queue
- `src-tauri/src/lib.rs` — `AppState` holds `param_tx: Mutex<Option<ParamSender>>`
- `src-tauri/src/commands/engine.rs` — `start_engine` creates param channel, stores tx in AppState, passes rx to CPAL; `set_parameter` now pushes to SPSC queue instead of directly locking engine; `get_audio_levels` command added
- `src-tauri/src/commands/ir.rs`, `midi.rs`, `scenes.rs`, `amp_presets.rs` — All engine parameter changes use `param_tx` via `set_parameter_value` for immediate HashMap consistency
- Frontend: `tauri.ts` — Added `getAudioLevels()` API, simulated levels with sine jitter

#### Real-time Audio Flow Visualization
- `crates/kicks-dsp/src/plugins.rs` — `PluginRegistry` tracks per-plugin `levels: Vec<f32>` updated every `process_all` cycle; `audio_levels()` returns them
- `frontend/src/stores/engineStore.ts` — Added `levels` state, `levelsError`, `pollLevels()` action (calls `getAudioLevels` at 50ms interval)
- `frontend/src/components/SignalChain/AudioFlow.tsx` — Full rewrite:
  - Replaced static `estimatedLevel()` guessing with real RMS bars from store polling
  - Gradient VU meter per slot (green → amber → red at standard thresholds)
  - dBFS label + percentage display, 0 dBFS clipping tickmark
  - Color zone legend (Safe / Warm / Hot)
  - Output level summary in header when engine runs

#### Settings Page — Device Dropdown + Auto-Restart
- `crates/kicks-dsp/src/audio_io.rs` — Added `DeviceInfo` struct and `list_audio_devices()` using CPAL `Host::devices()` enumeration, categorized as input/output
- `src-tauri/src/commands/settings.rs` — Added `list_audio_devices` Tauri command; `save_settings` now detects audio config changes and auto-restarts the CPAL stream with new sample rate / buffer size / device (engine state preserved)
- `frontend/src/types/tauri.ts` — Added `AudioDeviceInfo` interface
- `frontend/src/lib/tauri.ts` — Added `listAudioDevices()` API + simulated device list
- `frontend/src/pages/Settings.tsx` — Device Name text input replaced with populated `<select>` dropdown from CPAL enumeration; fetches devices on mount

### Files Most Likely Needed Next

1. **Flatpak / Snap packaging**
2. **User documentation**
3. **Build-time NAM inference support**

### Current State

- **54 tests pass**, `cargo check` and `npx tsc --noEmit` clean
- Build: `cargo build --release` produces the Tauri binary
- Dev mode: `cargo run` (or `cd frontend && npm run dev` for browser-only with simulated backend)
- All persistent state lives in `~/.config/kicks/`

## License

GNU General Public License v3.0 or later.
