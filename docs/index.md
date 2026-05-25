# Kicks Guitar Workstation — User Guide

**Kicks** is an open-source guitar & bass workstation for Linux (also macOS and
Windows via Tauri). It gives you a native DSP plugin chain — Boost, Amp/BassAmp,
Cab (with convolution IR), Delay, and Reverb — all running with low-latency
CPAL audio I/O (PipeWire/ALSA/CoreAudio/WASAPI).

## Quick Start

1. **Install** — see [Getting Started](#) for Flatpak, Snap, AppImage, or .deb
2. **Launch** — `kicks` from terminal or app launcher
3. **Start Engine** — press **Space** or click *Start Engine* on the Signal Chain page
4. **Play** — plug in your guitar/bass and hear the default chain (Boost → Amp → Cab → Delay → Reverb)
5. **Tweak** — drag to reorder pedals, click to adjust parameters
6. **Save** — save your tone as a preset for later

## Features

| Page | What you can do |
|------|----------------|
| [Signal Chain](signal-chain.md) | Build effect chains, reorder pedals, see real-time VU meters |
| [Presets](presets.md) | Save/load/manage presets organized by bank |
| [IR Browser](ir-browser.md) | Load impulse responses into the Cab plugin |
| [MIDI](midi.md) | Connect MIDI controllers, map CCs to parameters |
| [Live Mode](live-mode.md) | Manage scenes for live performance with next/prev navigation |
| [AI Assistant](ai-assistant.md) | Describe a tone and let AI generate the settings |
| [Settings](settings.md) | Configure audio device, engine mode, directories, AI provider |

## Plugins (Default Chain)

| Order | Plugin | What it does |
|-------|--------|-------------|
| 1 | **Boost** | Input gain stage (clean boost) |
| 2 | **Amp** | Pre-gain → 3-band EQ → waveshaper (distortion) → master volume. Toggle BassAmp mode for shifted EQ |
| 3 | **Cab** | Speaker cabinet sim: high-pass + low-pass filters, optional convolution IR |
| 4 | **Delay** | Digital delay with feedback and wet/dry mix |
| 5 | **Reverb** | Schroeder reverb (4 parallel combs → 2 series allpasses) with size, damping, mix |

## Reference

- [Keyboard Shortcuts](keyboard-shortcuts.md)
- [Troubleshooting](troubleshooting.md)
- [Settings Reference](settings.md)
