# Kicks Architecture

## Crate Hierarchy

```
┌─────────────────────────────────────────────────────┐
│                    Tauri Shell                        │
│                  src-tauri (app)                      │
│    Window management, IPC commands, menu, tray        │
│              Depends on: all crates                   │
└──────────────┬──────────────────────────────┬────────┘
               │                              │
               ▼                              ▼
┌──────────────────────┐    ┌──────────────────────────────┐
│   guitarix-rpc        │    │      kicks-dsp                │
│   JSON-RPC 2.0 client │    │  Real-time audio engine       │
│   Guitarix control     │    │  CPAL I/O + ringbuf + DSP     │
│   ─────────────────    │    │  ─────────────────            │
│   GuitarixClient       │    │  AudioEngine trait            │
│   Parameter queries    │    │  CpalAudioIO (full-duplex)    │
│   Preset management    │    │  JackAudioIO (feature-gated)  │
│   Bank operations      │    │  Boost, Amp, Cab, ...         │
└──────────────────────┘    └──────────────────────────────┘
               │                              │
               └──────────┬───────────────────┘
                          ▼
               ┌──────────────────────┐
               │     kicks-core        │
               │  Shared data models   │
               │  ─────────────────    │
               │  SignalChain model    │
               │  Preset/Bank types    │
               │  MIDI config types    │
               │  App config           │
               └──────────────────────┘
```

## Engine Modes

Kicks supports two audio processing backends, selectable at runtime:

1. **Guitarix Mode** — Routes audio through a running Guitarix engine via
   JSON-RPC. Kicks acts as a smart remote control. Requires `guitarix -N -p 4040`
   running in the background.

2. **Internal Mode** — Uses the kicks-dsp crate for native real-time processing.
   CPAL audio I/O (ALSA/PipeWire on Linux, CoreAudio on macOS) with embedded DSP
   plugins. JACK client also available as feature-gated alternative.

3. **Auto Mode** — Attempts internal first, falls back to Guitarix.

## Data Flow

```
User Input (UI)
     │
     ▼
Tauri IPC Commands (src-tauri/src/commands/)
     │
     ├── Guitarix Mode ──► guitarix-rpc ──► Guitarix Engine ──► JACK/PipeWire
     │
     └── Internal Mode ──► kicks-dsp (CpalAudioIO)
                             │
                    Input callback → ringbuf
                             │
                    Output callback → KicksEngine.process()
                             │
                    Speakers (ALSA/PipeWire/CoreAudio)
```

## Frontend Architecture

The frontend is a React + TypeScript SPA rendered in a Tauri webview.

```
frontend/src/
├── components/       # Reusable UI components
│   ├── SignalChain/  # Visual chain editor
│   ├── Presets/      # Preset browser
│   ├── IRBrowser/    # File browser for IRs/NAMs
│   ├── MIDI/         # MIDI configuration
│   ├── Live/         # Performance mode
│   ├── AI/           # Tone assistant panel
│   └── Common/       # Shared UI (knobs, sliders, etc.)
├── hooks/            # React hooks for Tauri invoke()
├── stores/           # State management
├── types/            # TypeScript type definitions
├── App.tsx           # Main app layout
└── main.tsx          # Entry point
```

## Key Design Decisions

- **Rust + Tauri** over Electron for memory efficiency and audio performance
- **Workspace crates** for clear separation between concerns
- **GPLv3** to align with Guitarix and the audio FOSS ecosystem
- **CPAL** as primary audio backend (ALSA/PipeWire on Linux, CoreAudio on macOS, WASAPI on Windows); JACK still available as feature-gated alternative
- **Demo-depth MVP** — prove all features work, deepen incrementally
