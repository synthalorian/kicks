# IR Browser

Impulse Responses (IRs) let you load real speaker cabinet captures into Kicks.
The Cab plugin supports both filter-only mode (built-in high-pass + low-pass)
and convolution IR mode for authentic cab sounds.

## Interface

- **Directory path** — shows the configured IR scan directory (configurable in Settings)
- **Scan** button — rescans the directory for WAV files
- **IR file list** — shows available IRs with metadata:
  - File name
  - Sample rate
  - Length (samples and milliseconds)
  - Channels (displayed before mono downmix)
- **Pick IR...** button — opens a native file dialog to load an IR from anywhere
- **Clear IR** button — removes the loaded IR, falling back to filter-only mode

## Loading an IR

1. Make sure your IR directory is configured in **Settings → File Directories**
2. Go to the **IR Browser** page
3. Click **Scan** to list available IRs
4. Click any IR in the list to load it into the Cab
5. The Cab now uses convolution processing with that IR

Alternatively, click **Pick IR...** to select a WAV file from anywhere on your filesystem.

## Supported Formats

| Format | Bit Depth | Notes |
|--------|-----------|-------|
| WAV | 16-bit int | Standard cab IR format |
| WAV | 24-bit int | Higher resolution |
| WAV | 32-bit int | Rare but supported |
| WAV | 32-bit float | Modern IR format |
| WAV | Multi-channel | Automatically downmixed to mono |

### Limitations

- Maximum IR length: **8192 samples** (≈185ms at 44.1kHz)
- Longer IRs are truncated — 8k samples is sufficient for accurate cab simulation
- Sample rate conversion is not performed; IR sample rate is stored as metadata

## Filter-Only Mode

When no IR is loaded, the Cab uses:

1. **High-pass filter** — cuts low frequencies (20-250 Hz range)
2. **Low-pass filter** — cuts high frequencies (2-8 kHz range)
3. **Level control** — output volume

This provides a basic cab sim even without IR files.

## IR Metadata

Once loaded, the Cab shows:
- IR file path
- Sample rate of the IR
- Length in samples and milliseconds
- File name

## Clear IR

Click **Clear IR** to remove the loaded convolution IR. The Cab falls back to
filter-only mode. Useful for A/B comparisons or if you want a synthetic cab sound.
