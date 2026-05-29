// Kicks — Tuner Plugin (YIN Pitch Detection)
//
// Real-time monophonic pitch detection using the YIN algorithm.
// Outputs the detected pitch via parameter queries; passes audio
// through with optional mute for silent tuning.
//
// YIN reference: de Cheveigné, A., & Kawahara, H. (2002).
// "YIN, a fundamental frequency estimator for speech and music."
// Journal of the Acoustical Society of America, 111(4), 1917-1930.

use crate::plugins::Plugin;

/// Buffer size for YIN analysis (covers ~150ms at 48kHz — enough for bass low E @ 41Hz).
const YIN_BUFFER_SIZE: usize = 4096;
/// Maximum detectable period in samples (~35 Hz at 48kHz — covers bass low E ≈41 Hz).
const YIN_MAX_PERIOD: usize = 1400;
/// Minimum detectable period in samples (~800 Hz at 48kHz).
const YIN_MIN_PERIOD: usize = 60;
/// YIN threshold for period detection.
const YIN_THRESHOLD: f32 = 0.1;

/// Note names for display.
const NOTE_NAMES: [&str; 12] = [
    "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B",
];

pub struct Tuner {
    /// Whether the tuner is actively processing (not bypassed).
    enabled: bool,
    /// Whether to mute the output (silent tuning).
    mute: bool,
    /// Sample rate in Hz.
    sample_rate: f32,
    /// Circular buffer of recent samples for YIN analysis.
    buffer: Vec<f32>,
    /// Write position in the circular buffer.
    write_pos: usize,
    /// Last detected frequency in Hz.
    detected_freq: f32,
    /// Confidence of the last detection (0..1).
    confidence: f32,
    /// Number of samples accumulated since last analysis.
    samples_since_analysis: usize,
    /// Analysis interval in samples (~20 Hz update rate).
    analysis_interval: usize,
    /// Cents deviation from the target note (-50..+50).
    cents_deviation: f32,
    /// Running sum for RMS level calculation.
    rms_sum: f32,
    /// RMS sample count.
    rms_count: usize,
    /// Pre-allocated scratch buffer for YIN contiguous samples (real-time safe).
    yin_samples: Vec<f32>,
    /// Pre-allocated scratch buffer for YIN difference function (real-time safe).
    yin_diff: Vec<f32>,
    /// Pre-allocated scratch buffer for YIN CMND (real-time safe).
    yin_cmnd: Vec<f32>,
}

impl Default for Tuner {
    fn default() -> Self {
        Self::new()
    }
}

impl Tuner {
    pub fn new() -> Self {
        Self {
            enabled: true,
            mute: false,
            sample_rate: 48000.0,
            buffer: vec![0.0f32; YIN_BUFFER_SIZE],
            write_pos: 0,
            detected_freq: 0.0,
            confidence: 0.0,
            samples_since_analysis: 0,
            analysis_interval: 2400, // ~50 Hz at 48k
            cents_deviation: 0.0,
            rms_sum: 0.0,
            rms_count: 0,
            yin_samples: vec![0.0f32; YIN_BUFFER_SIZE],
            yin_diff: vec![0.0f32; YIN_MAX_PERIOD],
            yin_cmnd: vec![0.0f32; YIN_MAX_PERIOD],
        }
    }

    /// Run the YIN algorithm on the current buffer and update detected pitch.
    /// Uses pre-allocated scratch buffers — no heap allocations in the real-time path.
    fn analyze_pitch(&mut self) {
        // Step 1: Extract contiguous samples from the circular buffer
        let samples = &mut self.yin_samples;
        for (i, sample) in samples.iter_mut().enumerate().take(YIN_BUFFER_SIZE) {
            let idx = (self.write_pos + i) % YIN_BUFFER_SIZE;
            *sample = self.buffer[idx];
        }

        // Step 2: Compute difference function
        let diff = &mut self.yin_diff;
        for tau in YIN_MIN_PERIOD..YIN_MAX_PERIOD {
            let mut sum = 0.0f32;
            let window = YIN_BUFFER_SIZE - YIN_MAX_PERIOD;
            for j in 0..window {
                let d = samples[j] - samples[j + tau];
                sum += d * d;
            }
            diff[tau] = sum;
        }

        // Step 3: Cumulative mean normalized difference
        let cmnd = &mut self.yin_cmnd;
        cmnd[0] = 1.0;
        let mut running_sum = 0.0f32;
        for tau in YIN_MIN_PERIOD..YIN_MAX_PERIOD {
            running_sum += diff[tau];
            cmnd[tau] = if running_sum > 0.0 {
                diff[tau] * (tau as f32) / running_sum
            } else {
                1.0
            };
        }

        // Step 4: Absolute threshold search
        let mut best_tau = 0usize;
        let mut min_cmnd = f32::MAX;
        for tau in YIN_MIN_PERIOD..(YIN_MAX_PERIOD - 1) {
            if cmnd[tau] < YIN_THRESHOLD {
                // Found a candidate — look for local minimum
                if cmnd[tau] < cmnd[tau + 1] {
                    best_tau = tau;
                    break;
                }
            }
            if cmnd[tau] < min_cmnd {
                min_cmnd = cmnd[tau];
                best_tau = tau;
            }
        }

        if best_tau == 0 {
            self.confidence = 0.0;
            self.detected_freq = 0.0;
            self.cents_deviation = 0.0;
            return;
        }

        // Step 5: Parabolic interpolation for finer period estimate
        let t0 = best_tau as f32;
        let y0 = cmnd[best_tau];
        let y1 = if best_tau + 1 < YIN_MAX_PERIOD {
            cmnd[best_tau + 1]
        } else {
            y0
        };
        let y_1 = if best_tau > 0 { cmnd[best_tau - 1] } else { y0 };

        let a = (y1 + y_1) / 2.0 - y0;
        let b = (y1 - y_1) / 2.0;
        let peak_offset = if a.abs() > 1e-6 { -b / (2.0 * a) } else { 0.0 };
        let period = t0 + peak_offset;

        // Step 6: Convert period to frequency
        let freq = self.sample_rate / period;

        // Step 7: Confidence based on CMND value
        let confidence = 1.0 - cmnd[best_tau].min(1.0);

        // Step 8: Compute cents deviation (heap-free for real-time safety)
        let cents = freq_to_cents(freq);

        self.detected_freq = freq;
        self.confidence = confidence;
        self.cents_deviation = cents;
    }

    /// Get the current detected frequency in Hz.
    pub fn frequency(&self) -> f32 {
        self.detected_freq
    }

    /// Get the current note name (computed on demand — no allocation in audio thread).
    pub fn note(&self) -> String {
        let (name, _) = freq_to_note(self.detected_freq);
        name
    }

    /// Get cents deviation (-50..+50).
    pub fn cents(&self) -> f32 {
        self.cents_deviation
    }

    /// Get detection confidence (0..1).
    pub fn detection_confidence(&self) -> f32 {
        self.confidence
    }
}

/// Convert a frequency to cents deviation from the nearest chromatic note.
/// No heap allocations — returns just the cents for real-time audio thread use.
fn freq_to_cents(freq: f32) -> f32 {
    if freq <= 0.0 {
        return 0.0;
    }
    // MIDI note number: 69 = A4 (440 Hz)
    let midi_note = 69.0 + 12.0 * (freq / 440.0).log2();
    let rounded_midi = midi_note.round();
    let cents = (midi_note - rounded_midi) * 100.0;
    cents.clamp(-50.0, 50.0)
}

/// Convert a frequency to a note name and cents deviation (allocates a String).
fn freq_to_note(freq: f32) -> (String, f32) {
    if freq <= 0.0 {
        return ("--".to_string(), 0.0);
    }
    let midi_note = 69.0 + 12.0 * (freq / 440.0).log2();
    let rounded_midi = midi_note.round();

    let note_idx = (rounded_midi as i32).rem_euclid(12) as usize;
    let octave = (rounded_midi as i32 / 12) - 1;

    let name = format!("{}{}", NOTE_NAMES[note_idx], octave);
    (name, freq_to_cents(freq))
}

impl Plugin for Tuner {
    fn name(&self) -> &str {
        "tuner"
    }

    fn init(&mut self, sample_rate: f64) -> anyhow::Result<()> {
        self.sample_rate = sample_rate as f32;
        self.analysis_interval = (sample_rate as usize) / 50; // ~50 Hz
        self.buffer.fill(0.0);
        self.write_pos = 0;
        Ok(())
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()> {
        if !self.enabled {
            output.copy_from_slice(input);
            return Ok(());
        }

        let mute = self.mute;

        for (i, &sample) in input.iter().enumerate() {
            // Write to analysis buffer
            self.buffer[self.write_pos] = sample;
            self.write_pos = (self.write_pos + 1) % YIN_BUFFER_SIZE;

            // Accumulate RMS
            self.rms_sum += sample * sample;
            self.rms_count += 1;

            self.samples_since_analysis += 1;

            // Run analysis at intervals
            if self.samples_since_analysis >= self.analysis_interval {
                // Only analyze if signal is strong enough (RMS > -40dB)
                let rms = if self.rms_count > 0 {
                    (self.rms_sum / self.rms_count as f32).sqrt()
                } else {
                    0.0
                };
                if rms > 0.01 {
                    self.analyze_pitch();
                } else {
                    self.confidence = 0.0;
                }
                self.samples_since_analysis = 0;
                self.rms_sum = 0.0;
                self.rms_count = 0;
            }

            // Output: muted or passthrough
            output[i] = if mute { 0.0 } else { sample };
        }
        Ok(())
    }

    fn get_parameter(&self, id: &str) -> Option<f32> {
        match id {
            "enabled" => Some(if self.enabled { 1.0 } else { 0.0 }),
            "mute" => Some(if self.mute { 1.0 } else { 0.0 }),
            "frequency" => Some(self.detected_freq),
            "confidence" => Some(self.confidence),
            "cents" => Some(self.cents_deviation),
            _ => None,
        }
    }

    fn set_parameter(&mut self, id: &str, value: f32) {
        match id {
            "enabled" => self.enabled = value > 0.5,
            "mute" => self.mute = value > 0.5,
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freq_to_note() {
        let (note, cents) = freq_to_note(440.0);
        assert_eq!(note, "A4");
        assert!(cents.abs() < 1.0);

        let (note, cents) = freq_to_note(880.0);
        assert_eq!(note, "A5");
        assert!(cents.abs() < 1.0);
    }

    #[test]
    fn test_tuner_passthrough() {
        let mut tuner = Tuner::new();
        tuner.init(48000.0).unwrap();
        let input = vec![0.5f32; 64];
        let mut output = vec![0.0; 64];
        tuner.process(&input, &mut output).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_tuner_mute() {
        let mut tuner = Tuner::new();
        tuner.init(48000.0).unwrap();
        tuner.set_parameter("mute", 1.0);
        let input = vec![0.5f32; 64];
        let mut output = vec![0.0; 64];
        tuner.process(&input, &mut output).unwrap();
        assert!(output.iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_tuner_detects_sine() {
        let mut tuner = Tuner::new();
        tuner.init(48000.0).unwrap();

        // Generate 440 Hz sine wave at 48kHz
        let freq = 440.0f32;
        let sr = 48000.0f32;
        let samples: Vec<f32> = (0..8192)
            .map(|i| (2.0 * std::f32::consts::PI * freq * i as f32 / sr).sin() * 0.5)
            .collect();

        let mut output = vec![0.0; 8192];
        tuner.process(&samples, &mut output).unwrap();

        // After processing enough samples, the tuner should have detected something
        assert!(tuner.frequency() > 0.0);
    }
}
