# Presets

The Presets page lets you save, load, and manage your tones organized by bank.

## Interface

- **Left panel** — bank list (e.g., "My Presets", "Jazz", "Metal", etc.)
- **Right panel** — preset list for the selected bank

## Loading a Preset

1. Click a bank on the left to select it
2. Click a preset in the list
3. The preset parameters are applied to the current signal chain

## Saving a Preset

1. Tweak your signal chain to get the tone you want
2. Click **Save Preset**
3. Enter a name and select the target bank
4. The current chain (all plugin types + parameters + order) is saved

## Organizing Presets

- **Rename** — click the rename icon on any preset
- **Delete** — click the delete icon (double-confirm to avoid accidents)
- Banks are managed automatically (they appear as you save presets to them)

## Built-in Presets

Kicks ships with **361 built-in presets** across 15 genres:

| Genre | Guitar | Bass |
|-------|--------|------|
| Rock | 25 | 5 |
| Metal | 30 | 6 |
| Blues | 25 | 5 |
| Jazz | 20 | 4 |
| Country | 20 | 4 |
| Funk | 15 | 5 |
| Pop | 25 | 5 |
| Punk | 15 | 3 |
| Ambient | 15 | 3 |
| Shred | 20 | 3 |
| Grunge | 15 | 3 |
| Doom | 15 | 4 |
| Nu-Metal | 15 | 3 |
| Djent | 15 | 4 |
| Stoner | 15 | 4 |

These are read-only presets included with the app. You can load them and then
save your modified version as a new preset.

## Persistence

All presets are stored in `~/.config/kicks/presets.json`.
