# Troubleshooting

## Audio

### "No default audio output device found"

Kicks can't find any audio hardware via CPAL.

**Solutions:**
- Make sure PipeWire or ALSA is running: `pactl info` or `aplay -l`
- Check that your audio interface is connected and powered
- On Linux: ensure your user has audio permissions (in the `audio` group)
- Try starting from terminal to see CPAL error messages

### Engine starts but no sound

**Solutions:**
- Check the device dropdown in **Settings → Audio Device** — make sure the
  correct output device is selected (not "System Default" if you have multiple)
- Lower the buffer size — large buffers on some drivers can appear silent
- Check your system volume/mute controls: `pavucontrol`, `alsamixer`, etc.
- Verify the master volume and gain on each plugin aren't at zero
- Check the AudioFlow VU meters — if they show signal, audio is reaching the
  engine but not your speakers

### Audio crackling or dropouts

**Solutions:**
- Increase the **Buffer Size** in Settings (try 512 or 1024)
- Close other audio applications
- Check CPU usage — the DSP chain is efficient but heavy plugin chains add up
- Try a different sample rate (48000 is the safest default)

### Engine fails to start

Check the terminal output for errors. Common causes:

- Invalid audio device name in config
- Sample rate not supported by hardware
- Buffer size too small for the hardware/driver
- Another application has exclusive access to the audio device

## Impulse Responses

### IR won't load

Kicks supports 16/24/32-bit WAV files (int or float). Check:

- Is the file a valid WAV? Try opening it in another audio app
- Is it longer than 8192 samples? (~185ms at 44.1kHz) — Kicks truncates longer IRs
- Is the file path readable? Check file permissions

### IR sounds wrong

- IRs recorded at very different sample rates may sound off
- Cabinet IRs should be mono — Kicks auto-downmixes stereo to mono
- Try a different IR — some captures are better than others

## MIDI

### Device not appearing in list

- Check physical connection: `lsusb` | `aconnect -i` (ALSA) | `pw-cli list-objects`
- Your user needs permissions for the MIDI device (usually in `audio` group)
- Some USB MIDI devices need to be plugged in before launching Kicks
- Try re-scanning or restarting Kicks

### MIDI Learn not working

1. Click the parameter in the UI first (it should highlight)
2. Then move your MIDI control
3. Make sure the device is connected (shows as connected in the MIDI page)

## AI Assistant

### "API error" or timeout

- Check your API key is correct in Settings
- Verify the endpoint URL matches your provider
- For local models (Ollama): make sure the server is running and reachable
- Some models don't follow structured output well — try a different model

## Flatpak / Sandbox Issues

If running as a Flatpak:

- Presets and config go in `~/.var/app/com.kicks.guitar-workstation/config/kicks/`
- MIDI devices need host access — ensure `--socket=session-bus` is set
- Audio devices need PulseAudio socket access
- If IR files are on an external drive, make sure it's accessible to the sandbox

## Getting Help

If you run into something not covered here, check the terminal output for
tracing logs. Kicks uses structured logging — you can increase verbosity:

```bash
KICKS_LOG=debug kicks
```
