# Live Mode

Live Mode (Scenes) lets you manage snapshots of your signal chain for live
performance. Each scene stores the complete chain — plugin types, parameters,
order, enable states, and wet/dry mix.

## Interface

- **Scene list** — all saved scenes shown as a list with drag handles
- **Save Current** — saves the current signal chain as a new scene
- **Load** — restores a scene's chain (overwrites current chain)
- **Next / Prev** — navigate scenes in order (great for footswitch control)
- **Rename** / **Delete** — manage scenes

## Creating a Scene

1. Dial in your tone on the Signal Chain page
2. Go to **Live Mode**
3. Click **Save Current**
4. Name your scene
5. The current chain is captured and added to the scene list

## Loading a Scene

- Click **Load** on any scene to restore its signal chain
- The engine continues running — seamless transition
- Use **Next** / **Prev** to step through scenes in order

## Reordering Scenes

Drag the handle on any scene to reorder the list. Order matters for next/prev
navigation during a performance.

## Performance Flow

```
Set 1:
   Scene 1: Clean rhythm  → Scene 2: Lead solo  → Scene 3: Crunch chords
                               ↓
                      Press "Next" to advance
```

You can assign MIDI CCs to scene navigation for hands-free control.

## Persistence

All scenes are stored in `~/.config/kicks/scenes.json`.

## Tips

- Use scenes to switch between verse/chorus/bridge tones
- Create scenes for each song in your setlist
- Order them in performance sequence for next/prev navigation
- Combine with MIDI foot controller for hands-free switching
