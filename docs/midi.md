# MIDI Configuration

Kicks integrates with hardware MIDI controllers so you can control parameters
with footswitches, expression pedals, and knob controllers in real time.

## Device Setup

1. Connect your MIDI controller via USB (or MIDI interface)
2. Go to the **MIDI** page
3. The device list auto-populates with detected ports
4. Click **Connect** on your device
5. The device shows as connected — MIDI events are now being received

### Disconnecting

Click **Disconnect** on an active device to stop receiving its events.

## MIDI Learn

MIDI Learn lets you map a hardware control to any plugin parameter by simply
moving the control:

1. Click **MIDI Learn** (or the learn toggle button)
2. Click the parameter in the UI you want to map
3. Move the hardware knob/fader/pedal on your MIDI controller
4. The mapping is created automatically

## Manual Mapping

The mapping table shows all configured mappings:

| Column | Description |
|--------|-------------|
| CC# | MIDI Control Change number |
| Channel | MIDI channel (1-16) |
| Parameter | Target parameter ID (e.g. `amp.gain`, `reverb.mix`) |
| Min | Minimum value (0.0-1.0) |
| Max | Maximum value (0.0-1.0) |

You can manually add, edit, or delete mappings from this table.

## Mapping Ranges

Each mapping has a **min** and **max** range. This lets you:

- Limit a foot pedal's sweep to just the usable range
- Invert a control (swap min/max)
- Set a fixed value by setting min = max

The MIDI CC value (0-127) is linearly mapped to the parameter's 0.0-1.0 range,
clamped by the mapping's min/max.

## Persistence

MIDI configuration (device selections + all CC mappings) is saved to
`~/.config/kicks/midi_config.json`. It's restored automatically on next launch.

## Real-Time Performance

MIDI CC events flow through the **lock-free SPSC parameter channel** — the same
channel used by the UI for parameter changes. This means:

- MIDI changes never block the audio callback
- They're applied before the next audio processing cycle
- There's zero risk of audio glitches from MIDI/mapping operations

## Troubleshooting

| Symptom | Likely Cause |
|---------|-------------|
| Device not listed | Not connected, or no permissions. Try `lsusb` to verify |
| MIDI Learn not registering | Make sure you clicked the parameter first, then move the control |
| Parameter jumps to wrong value | Check min/max mapping range |
| Device disconnects | Try a different USB port or cable |
