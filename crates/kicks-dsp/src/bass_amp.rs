// Kicks — BassAmp Plugin
//
// A dedicated bass guitar amplifier simulation with:
//   • Extended low-end shelving (20–150 Hz)
//   • A dedicated compressor for bass dynamics
//   • A bass-optimized waveshaper (softer knee than guitar amp)
//   • Shifted EQ frequencies appropriate for bass (100/500/2500 Hz)
//   • Optional sub-harmonic enhancement

use std::f32::consts::PI;

use crate::plugins::Plugin;

// ═══════════════════════════════════════════════════════════════════════════════
// DSP Utilities (local copies for self-containment)
// ═══════════════════════════════════════════════════════════════════════════════

struct BiquadFilter {
    b0: f32,
    b1: f32,
    b2: f32,
    a1: f32,
    a2: f32,
    x1: f32,
    x2: f32,
    y1: f32,
    y2: f32,
}

impl BiquadFilter {
    fn process(&mut self, sample: f32) -> f32 {
        let out = self.b0 * sample + self.b1 * self.x1 + self.b2 * self.x2
            - self.a1 * self.y1
            - self.a2 * self.y2;
        self.x2 = self.x1;
        self.x1 = sample;
        self.y2 = self.y1;
        self.y1 = out;
        out
    }

    fn design_low_shelf(sr: f32, gain_db: f32, freq: f32, q: f32) -> Self {
        let a = 10_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq / sr;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);
        let sqrt_2a = (2.0 * a).sqrt();

        let b0 = a * ((a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * sqrt_2a * alpha);
        let b1 = 2.0 * a * ((a - 1.0) - (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * sqrt_2a * alpha);
        let a0 = (a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * sqrt_2a * alpha;
        let a1 = -2.0 * ((a - 1.0) + (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * sqrt_2a * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    fn design_high_shelf(sr: f32, gain_db: f32, freq: f32, q: f32) -> Self {
        let a = 10_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq / sr;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);
        let sqrt_2a = (2.0 * a).sqrt();

        let b0 = a * ((a + 1.0) + (a - 1.0) * cos_w0 + 2.0 * sqrt_2a * alpha);
        let b1 = -2.0 * a * ((a - 1.0) + (a + 1.0) * cos_w0);
        let b2 = a * ((a + 1.0) + (a - 1.0) * cos_w0 - 2.0 * sqrt_2a * alpha);
        let a0 = (a + 1.0) - (a - 1.0) * cos_w0 + 2.0 * sqrt_2a * alpha;
        let a1 = 2.0 * ((a - 1.0) - (a + 1.0) * cos_w0);
        let a2 = (a + 1.0) - (a - 1.0) * cos_w0 - 2.0 * sqrt_2a * alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    fn design_peaking(sr: f32, gain_db: f32, freq: f32, q: f32) -> Self {
        let a = 10_f32.powf(gain_db / 40.0);
        let w0 = 2.0 * PI * freq / sr;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);

        let b0 = 1.0 + alpha * a;
        let b1 = -2.0 * cos_w0;
        let b2 = 1.0 - alpha * a;
        let a0 = 1.0 + alpha / a;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha / a;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }

    fn design_lowpass(sr: f32, cutoff: f32, q: f32) -> Self {
        let w0 = 2.0 * PI * cutoff / sr;
        let sin_w0 = w0.sin();
        let cos_w0 = w0.cos();
        let alpha = sin_w0 / (2.0 * q);

        let b0 = (1.0 - cos_w0) / 2.0;
        let b1 = 1.0 - cos_w0;
        let b2 = (1.0 - cos_w0) / 2.0;
        let a0 = 1.0 + alpha;
        let a1 = -2.0 * cos_w0;
        let a2 = 1.0 - alpha;

        Self {
            b0: b0 / a0,
            b1: b1 / a0,
            b2: b2 / a0,
            a1: a1 / a0,
            a2: a2 / a0,
            x1: 0.0,
            x2: 0.0,
            y1: 0.0,
            y2: 0.0,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Compressor
// ═══════════════════════════════════════════════════════════════════════════════

/// A simple feedback-style peak compressor.
/// Threshold, ratio, attack, release — all real-time safe.
struct Compressor {
    threshold: f32, // dB (negative)
    ratio: f32,     // e.g. 4.0 = 4:1
    attack: f32,    // ms
    release: f32,   // ms
    sample_rate: f32,
    /// Envelope follower (linear).
    envelope: f32,
    /// Attack coefficient.
    attack_coef: f32,
    /// Release coefficient.
    release_coef: f32,
}

impl Compressor {
    fn new(sample_rate: f32) -> Self {
        let mut c = Self {
            threshold: -20.0,
            ratio: 4.0,
            attack: 5.0,
            release: 100.0,
            sample_rate,
            envelope: 0.0,
            attack_coef: 0.0,
            release_coef: 0.0,
        };
        c.update_coeffs();
        c
    }

    fn update_coeffs(&mut self) {
        // Convert ms to per-sample coefficient
        self.attack_coef = (-1000.0 / (self.attack * self.sample_rate)).exp();
        self.release_coef = (-1000.0 / (self.release * self.sample_rate)).exp();
    }

    fn set_threshold(&mut self, db: f32) {
        self.threshold = db.clamp(-60.0, 0.0);
    }

    fn set_ratio(&mut self, ratio: f32) {
        self.ratio = ratio.clamp(1.0, 20.0);
    }

    #[allow(dead_code)]
    fn set_attack(&mut self, ms: f32) {
        self.attack = ms.clamp(0.1, 500.0);
        self.update_coeffs();
    }

    #[allow(dead_code)]
    fn set_release(&mut self, ms: f32) {
        self.release = ms.clamp(1.0, 2000.0);
        self.update_coeffs();
    }

    fn process(&mut self, sample: f32) -> f32 {
        let input_level = sample.abs();
        // Envelope follower
        if input_level > self.envelope {
            self.envelope =
                self.attack_coef * self.envelope + (1.0 - self.attack_coef) * input_level;
        } else {
            self.envelope =
                self.release_coef * self.envelope + (1.0 - self.release_coef) * input_level;
        }

        // Convert to dB
        let env_db = if self.envelope > 1e-10 {
            20.0 * self.envelope.log10()
        } else {
            -120.0
        };

        // Compute gain reduction
        let gain_db = if env_db > self.threshold {
            (self.threshold - env_db) * (1.0 - 1.0 / self.ratio)
        } else {
            0.0
        };

        let gain_linear = 10_f32.powf(gain_db / 20.0);
        sample * gain_linear
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Bass-optimized Waveshaper
// ═══════════════════════════════════════════════════════════════════════════════

/// A softer, more musical waveshaper optimized for bass frequencies.
/// Uses a blend of tube-like even harmonics and gentle limiting.
fn bass_waveshape(sample: f32, drive: f32) -> f32 {
    let x = sample * drive;
    // Soft-knee tanh for warmth
    let soft = x.tanh();
    // Slight asymmetry for even harmonics
    let asym = (x + 0.1 * x * x).tanh();
    // Blend: mostly soft, slight asymmetry
    soft * 0.85 + asym * 0.15
}

// ═══════════════════════════════════════════════════════════════════════════════
// Sub-Harmonic Generator
// ═══════════════════════════════════════════════════════════════════════════════

/// Generates a sub-octave signal by full-wave rectification + LP filtering.
struct SubHarmonic {
    lp_filter: BiquadFilter,
}

impl SubHarmonic {
    fn new(sample_rate: f32) -> Self {
        Self {
            lp_filter: BiquadFilter::design_lowpass(sample_rate, 120.0, 0.7),
        }
    }

    fn process(&mut self, sample: f32) -> f32 {
        // Full-wave rectification (creates sub-harmonic content)
        let rectified = sample.abs();
        // Lowpass to isolate the sub-harmonic
        self.lp_filter.process(rectified)
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BassAmp Plugin
// ═══════════════════════════════════════════════════════════════════════════════

pub struct BassAmp {
    gain: f32,
    master: f32,
    bass: f32,
    mid: f32,
    treble: f32,
    drive: f32,
    /// Compressor threshold (0..1 → -40..0 dB)
    comp_threshold: f32,
    /// Compressor ratio (0..1 → 1:1..10:1)
    comp_ratio: f32,
    /// Sub-harmonic mix (0..1)
    sub_mix: f32,
    sample_rate: f32,
    bass_filter: Option<BiquadFilter>,
    mid_filter: Option<BiquadFilter>,
    treble_filter: Option<BiquadFilter>,
    compressor: Option<Compressor>,
    sub_harmonic: Option<SubHarmonic>,
    params_dirty: bool,
}

impl Default for BassAmp {
    fn default() -> Self {
        Self::new()
    }
}

impl BassAmp {
    pub fn new() -> Self {
        Self {
            gain: 0.4,
            master: 0.7,
            bass: 0.6,
            mid: 0.5,
            treble: 0.4,
            drive: 0.3,
            comp_threshold: 0.5, // -20 dB
            comp_ratio: 0.3,     // ~3:1
            sub_mix: 0.0,
            sample_rate: 48000.0,
            bass_filter: None,
            mid_filter: None,
            treble_filter: None,
            compressor: None,
            sub_harmonic: None,
            params_dirty: true,
        }
    }

    fn recalc_filters(&mut self) {
        let sr = self.sample_rate;
        // Bass-appropriate EQ frequencies
        let bass_db = (self.bass * 24.0) - 12.0;
        let mid_db = (self.mid * 24.0) - 12.0;
        let treble_db = (self.treble * 24.0) - 12.0;

        self.bass_filter = Some(BiquadFilter::design_low_shelf(sr, bass_db, 100.0, 0.7));
        self.mid_filter = Some(BiquadFilter::design_peaking(sr, mid_db, 500.0, 0.7));
        self.treble_filter = Some(BiquadFilter::design_high_shelf(sr, treble_db, 2500.0, 0.7));
    }

    fn drive_gain(&self) -> f32 {
        1.0 + self.drive * 9.0 // 1..10
    }
}

impl Plugin for BassAmp {
    fn name(&self) -> &str {
        "bass_amp"
    }

    fn init(&mut self, sample_rate: f64) -> anyhow::Result<()> {
        self.sample_rate = sample_rate as f32;
        self.compressor = Some(Compressor::new(self.sample_rate));
        self.sub_harmonic = Some(SubHarmonic::new(self.sample_rate));
        self.params_dirty = true;
        Ok(())
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()> {
        if self.params_dirty {
            self.recalc_filters();
            self.params_dirty = false;
        }

        let pre_gain = self.gain * 10.0;
        let drive = self.drive_gain();
        let master = self.master;
        let sub_mix = self.sub_mix;

        let bass = self.bass_filter.as_mut().unwrap();
        let mid = self.mid_filter.as_mut().unwrap();
        let treble = self.treble_filter.as_mut().unwrap();
        let comp = self.compressor.as_mut().unwrap();
        let sub = self.sub_harmonic.as_mut().unwrap();

        // Update compressor from parameters
        let threshold_db = -40.0 + self.comp_threshold * 40.0; // 0..1 → -40..0
        let ratio = 1.0 + self.comp_ratio * 9.0; // 0..1 → 1..10
        comp.set_threshold(threshold_db);
        comp.set_ratio(ratio);

        for (i, sample) in input.iter().enumerate() {
            // Pre-gain
            let mut s = sample * pre_gain;

            // Sub-harmonic enhancement (before EQ)
            let sub_signal = sub.process(s) * sub_mix;
            s += sub_signal;

            // 3-band EQ
            s = bass.process(s);
            s = mid.process(s);
            s = treble.process(s);

            // Compression
            s = comp.process(s);

            // Distortion stage (softer than guitar amp)
            s = bass_waveshape(s, drive);

            // Master volume
            s *= master;

            output[i] = s;
        }
        Ok(())
    }

    fn get_parameter(&self, id: &str) -> Option<f32> {
        match id {
            "gain" => Some(self.gain),
            "master" => Some(self.master),
            "bass" => Some(self.bass),
            "mid" => Some(self.mid),
            "treble" => Some(self.treble),
            "drive" => Some(self.drive),
            "comp_threshold" => Some(self.comp_threshold),
            "comp_ratio" => Some(self.comp_ratio),
            "sub_mix" => Some(self.sub_mix),
            _ => None,
        }
    }

    fn set_parameter(&mut self, id: &str, value: f32) {
        let v = value.clamp(0.0, 1.0);
        match id {
            "gain" => self.gain = v,
            "master" => self.master = v,
            "bass" => {
                self.bass = v;
                self.params_dirty = true;
            }
            "mid" => {
                self.mid = v;
                self.params_dirty = true;
            }
            "treble" => {
                self.treble = v;
                self.params_dirty = true;
            }
            "drive" => self.drive = v,
            "comp_threshold" => self.comp_threshold = v,
            "comp_ratio" => self.comp_ratio = v,
            "sub_mix" => self.sub_mix = v,
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

// ═══════════════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    fn test_buffer(size: usize) -> Vec<f32> {
        (0..size)
            .map(|i| (i as f32 / size as f32) * 0.5 - 0.25)
            .collect()
    }

    #[test]
    fn test_bass_amp_processes() {
        let mut amp = BassAmp::new();
        amp.init(48000.0).unwrap();
        let input = test_buffer(64);
        let mut output = vec![0.0; 64];
        amp.process(&input, &mut output).unwrap();
        assert!(output.iter().any(|&x| x != 0.0));
    }

    #[test]
    fn test_bass_amp_parameters() {
        let mut amp = BassAmp::new();
        amp.set_parameter("gain", 1.0);
        assert!((amp.get_parameter("gain").unwrap() - 1.0).abs() < 1e-6);
        amp.set_parameter("comp_threshold", 0.7);
        assert!((amp.get_parameter("comp_threshold").unwrap() - 0.7).abs() < 1e-6);
        amp.set_parameter("sub_mix", 0.5);
        assert!((amp.get_parameter("sub_mix").unwrap() - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_compressor() {
        let mut comp = Compressor::new(48000.0);
        comp.set_threshold(-20.0);
        comp.set_ratio(4.0);

        // Quiet signal should pass through
        let quiet = 0.01f32;
        let out_quiet = comp.process(quiet);
        assert!((out_quiet - quiet).abs() < 0.001);

        // Feed a sequence of loud samples to let the envelope build up
        let loud = 0.8f32;
        let mut out_loud = loud;
        for _ in 0..100 {
            out_loud = comp.process(loud);
        }
        assert!(out_loud < loud);
    }

    #[test]
    fn test_sub_harmonic() {
        let mut sub = SubHarmonic::new(48000.0);
        // A sine-like input should produce some output after rectification
        let input: Vec<f32> = (0..64).map(|i| (i as f32 * 0.1).sin()).collect();
        let mut output = vec![0.0f32; 64];
        for (i, &sample) in input.iter().enumerate() {
            output[i] = sub.process(sample);
        }
        assert!(output.iter().any(|&x| x != 0.0));
    }
}
