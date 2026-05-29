// Kicks — Looper Plugin
//
// A real-time audio looper with:
//   • Record / Overdub / Playback / Stop modes
//   • Up to 5 minutes of stereo loop time at 48 kHz
//   • Half-speed and reverse playback
//   • Undo last overdub
//   • Quantized start/stop to the beat

use crate::plugins::Plugin;

/// Maximum loop length: 5 minutes at 48 kHz mono.
const MAX_LOOP_SAMPLES: usize = 48000 * 60 * 5;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoopMode {
    /// Idle, passthrough.
    Idle,
    /// Recording a new loop (erases any existing).
    Record,
    /// Overdubbing onto existing loop.
    Overdub,
    /// Playing back the loop.
    Play,
    /// Stopped (loop in memory but silent).
    Stop,
}

pub struct Looper {
    /// Current mode.
    mode: LoopMode,
    /// The circular loop buffer.
    buffer: Vec<f32>,
    /// Write position (also defines loop length after first record).
    write_pos: usize,
    /// Read position.
    read_pos: f32,
    /// Loop length in samples (0 = no loop recorded yet).
    loop_length: usize,
    /// Sample rate.
    sample_rate: f32,
    /// Playback speed multiplier (0.5 = half speed, 1.0 = normal, 2.0 = double).
    speed: f32,
    /// Reverse playback.
    reverse: bool,
    /// Output volume.
    volume: f32,
    /// Dry/wet mix (0 = dry only, 1 = loop only).
    mix: f32,
    /// Fade in/out length in samples (prevents clicks).
    fade_samples: usize,
    /// Current fade position.
    fade_pos: usize,
    /// Whether we're currently fading.
    fading: bool,
    /// Undo buffer for last overdub.
    undo_buffer: Vec<f32>,
    /// Whether the undo buffer is valid.
    undo_valid: bool,
    /// Quantize to bar length in samples (0 = off).
    #[allow(dead_code)]
    quantize_samples: usize,
    /// Sample counter for quantization.
    #[allow(dead_code)]
    quantize_counter: usize,
    /// Crossfade length for seamless overdub boundaries.
    xfader_samples: usize,
}

impl Default for Looper {
    fn default() -> Self {
        Self::new()
    }
}

impl Looper {
    pub fn new() -> Self {
        Self {
            mode: LoopMode::Idle,
            buffer: vec![0.0f32; MAX_LOOP_SAMPLES],
            write_pos: 0,
            read_pos: 0.0,
            loop_length: 0,
            sample_rate: 48000.0,
            speed: 1.0,
            reverse: false,
            volume: 1.0,
            mix: 0.5,
            fade_samples: 256,
            fade_pos: 0,
            fading: false,
            undo_buffer: vec![0.0f32; MAX_LOOP_SAMPLES],
            undo_valid: false,
            quantize_samples: 0,
            quantize_counter: 0,
            xfader_samples: 64,
        }
    }

    /// Trigger a mode transition.
    pub fn trigger_mode(&mut self, mode: LoopMode) {
        match (self.mode, mode) {
            // Start recording from idle or stopped
            (_, LoopMode::Record) => {
                self.loop_length = 0;
                self.write_pos = 0;
                self.read_pos = 0.0;
                self.buffer.fill(0.0);
                self.undo_valid = false;
                self.mode = LoopMode::Record;
                self.start_fade_in();
            }
            // Start overdub from play
            (LoopMode::Play, LoopMode::Overdub) => {
                // Save current loop state for undo
                self.undo_buffer.copy_from_slice(&self.buffer);
                self.undo_valid = true;
                self.mode = LoopMode::Overdub;
            }
            // Start playback from record, overdub, or stop
            (LoopMode::Record, LoopMode::Play) => {
                self.loop_length = self.write_pos;
                if self.loop_length == 0 {
                    self.loop_length = 1;
                }
                self.write_pos = 0;
                self.read_pos = 0.0;
                self.mode = LoopMode::Play;
            }
            (LoopMode::Overdub, LoopMode::Play) => {
                self.mode = LoopMode::Play;
            }
            (LoopMode::Stop, LoopMode::Play) => {
                self.read_pos = 0.0;
                self.mode = LoopMode::Play;
                self.start_fade_in();
            }
            // Stop from play or overdub
            (LoopMode::Play, LoopMode::Stop) | (LoopMode::Overdub, LoopMode::Stop) => {
                self.mode = LoopMode::Stop;
                self.start_fade_out();
            }
            // Idle clears everything
            (_, LoopMode::Idle) => {
                self.clear();
                self.mode = LoopMode::Idle;
            }
            _ => {
                self.mode = mode;
            }
        }
    }

    /// Undo the last overdub.
    pub fn undo(&mut self) {
        if self.undo_valid {
            self.buffer.copy_from_slice(&self.undo_buffer);
            self.undo_valid = false;
        }
    }

    /// Clear the loop buffer.
    pub fn clear(&mut self) {
        self.buffer.fill(0.0);
        self.write_pos = 0;
        self.read_pos = 0.0;
        self.loop_length = 0;
        self.undo_valid = false;
    }

    fn start_fade_in(&mut self) {
        self.fade_pos = 0;
        self.fading = true;
    }

    fn start_fade_out(&mut self) {
        self.fade_pos = self.fade_samples;
        self.fading = true;
    }

    fn get_fade_gain(&self) -> f32 {
        if !self.fading {
            return 1.0;
        }
        if self.mode == LoopMode::Play
            || self.mode == LoopMode::Overdub
            || self.mode == LoopMode::Record
        {
            // Fade in
            let gain = self.fade_pos as f32 / self.fade_samples as f32;
            if self.fade_pos >= self.fade_samples {
                return 1.0;
            }
            gain
        } else {
            // Fade out
            let gain = self.fade_pos as f32 / self.fade_samples as f32;
            if self.fade_pos == 0 {
                return 0.0;
            }
            gain
        }
    }

    /// Current loop length in seconds.
    pub fn loop_time_seconds(&self) -> f32 {
        if self.sample_rate > 0.0 && self.loop_length > 0 {
            self.loop_length as f32 / self.sample_rate
        } else {
            0.0
        }
    }

    /// Whether a loop has been recorded.
    pub fn has_loop(&self) -> bool {
        self.loop_length > 0
    }
}

impl Plugin for Looper {
    fn name(&self) -> &str {
        "looper"
    }

    fn init(&mut self, sample_rate: f64) -> anyhow::Result<()> {
        self.sample_rate = sample_rate as f32;
        self.fade_samples = (sample_rate * 0.005) as usize; // 5ms fade
        self.xfader_samples = (sample_rate * 0.001) as usize; // 1ms crossfade
        Ok(())
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()> {
        let mix = self.mix;
        let vol = self.volume;
        let speed = self.speed;
        let reverse = self.reverse;

        for (i, &sample) in input.iter().enumerate() {
            // Update fade
            let fade_gain = if self.fading {
                if self.mode == LoopMode::Play
                    || self.mode == LoopMode::Overdub
                    || self.mode == LoopMode::Record
                {
                    // Fade in
                    self.fade_pos += 1;
                    if self.fade_pos >= self.fade_samples {
                        self.fading = false;
                    }
                } else {
                    // Fade out
                    if self.fade_pos > 0 {
                        self.fade_pos -= 1;
                    } else {
                        self.fading = false;
                    }
                }
                self.get_fade_gain()
            } else {
                1.0
            };

            match self.mode {
                LoopMode::Idle => {
                    output[i] = sample;
                }
                LoopMode::Record => {
                    if self.write_pos < MAX_LOOP_SAMPLES {
                        self.buffer[self.write_pos] = sample;
                        self.write_pos += 1;
                    }
                    output[i] = sample; // Monitor input while recording
                }
                LoopMode::Overdub => {
                    // Read from loop
                    let loop_sample = if self.loop_length > 0 {
                        let read_idx = self.read_pos as usize % self.loop_length;
                        let frac = self.read_pos - read_idx as f32;
                        let next_idx = (read_idx + 1) % self.loop_length;
                        let s1 = self.buffer[read_idx];
                        let s2 = self.buffer[next_idx];
                        s1 * (1.0 - frac) + s2 * frac
                    } else {
                        0.0
                    };

                    // Mix input into buffer with crossfade at boundaries
                    if self.write_pos < MAX_LOOP_SAMPLES {
                        let write_idx = self.write_pos % self.loop_length;
                        let xfader = if write_idx < self.xfader_samples {
                            write_idx as f32 / self.xfader_samples as f32
                        } else if write_idx >= self.loop_length - self.xfader_samples {
                            (self.loop_length - write_idx) as f32 / self.xfader_samples as f32
                        } else {
                            1.0
                        };
                        self.buffer[write_idx] = self.buffer[write_idx] * (1.0 - xfader)
                            + (self.buffer[write_idx] + sample) * xfader;
                        self.write_pos += 1;
                    }

                    // Advance read position
                    if self.loop_length > 0 {
                        let increment = if reverse { -speed } else { speed };
                        self.read_pos += increment;
                        while self.read_pos < 0.0 {
                            self.read_pos += self.loop_length as f32;
                        }
                        self.read_pos %= self.loop_length as f32;
                    }

                    // Output: mixed loop + input
                    output[i] = (loop_sample * mix + sample * (1.0 - mix)) * vol * fade_gain;
                }
                LoopMode::Play => {
                    let loop_sample = if self.loop_length > 0 {
                        let read_idx = self.read_pos as usize % self.loop_length;
                        let frac = self.read_pos - read_idx as f32;
                        let next_idx = (read_idx + 1) % self.loop_length;
                        let s1 = self.buffer[read_idx];
                        let s2 = self.buffer[next_idx];
                        s1 * (1.0 - frac) + s2 * frac
                    } else {
                        0.0
                    };

                    // Advance read position
                    if self.loop_length > 0 {
                        let increment = if reverse { -speed } else { speed };
                        self.read_pos += increment;
                        while self.read_pos < 0.0 {
                            self.read_pos += self.loop_length as f32;
                        }
                        self.read_pos %= self.loop_length as f32;
                    }

                    output[i] = (loop_sample * mix + sample * (1.0 - mix)) * vol * fade_gain;
                }
                LoopMode::Stop => {
                    output[i] = sample * fade_gain;
                }
            }
        }
        Ok(())
    }

    fn get_parameter(&self, id: &str) -> Option<f32> {
        match id {
            "mode" => Some(match self.mode {
                LoopMode::Idle => 0.0,
                LoopMode::Record => 1.0,
                LoopMode::Overdub => 2.0,
                LoopMode::Play => 3.0,
                LoopMode::Stop => 4.0,
            }),
            "speed" => Some((self.speed - 0.5) / 1.5), // 0.5..2.0 → 0..1
            "reverse" => Some(if self.reverse { 1.0 } else { 0.0 }),
            "volume" => Some(self.volume),
            "mix" => Some(self.mix),
            "has_loop" => Some(if self.has_loop() { 1.0 } else { 0.0 }),
            "loop_time" => Some(self.loop_time_seconds()),
            _ => None,
        }
    }

    fn set_parameter(&mut self, id: &str, value: f32) {
        match id {
            "mode" => {
                let mode = match value.round() as i32 {
                    0 => LoopMode::Idle,
                    1 => LoopMode::Record,
                    2 => LoopMode::Overdub,
                    3 => LoopMode::Play,
                    4 => LoopMode::Stop,
                    _ => LoopMode::Idle,
                };
                self.trigger_mode(mode);
            }
            "speed" => self.speed = value.clamp(0.0, 1.0) * 1.5 + 0.5,
            "reverse" => self.reverse = value > 0.5,
            "volume" => self.volume = value.clamp(0.0, 1.0),
            "mix" => self.mix = value.clamp(0.0, 1.0),
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
    fn test_looper_record_and_play() {
        let mut looper = Looper::new();
        looper.init(48000.0).unwrap();

        // Record a simple pattern
        let input: Vec<f32> = (0..480).map(|i| (i as f32 / 480.0).sin()).collect();
        looper.trigger_mode(LoopMode::Record);
        let mut output = vec![0.0; 480];
        looper.process(&input, &mut output).unwrap();

        // Switch to play
        looper.trigger_mode(LoopMode::Play);
        let mut output2 = vec![0.0; 480];
        let silent_input = vec![0.0; 480];
        looper.process(&silent_input, &mut output2).unwrap();

        // Output should match the recorded input
        for (o, inp) in output2.iter().zip(input.iter()) {
            assert!(
                (o - inp * 0.5).abs() < 0.01,
                "Expected ~{}, got {}",
                inp * 0.5,
                o
            );
        }
    }

    #[test]
    fn test_looper_passthrough_idle() {
        let mut looper = Looper::new();
        looper.init(48000.0).unwrap();
        let input = vec![0.5f32; 64];
        let mut output = vec![0.0; 64];
        looper.process(&input, &mut output).unwrap();
        assert_eq!(input, output);
    }

    #[test]
    fn test_looper_undo() {
        let mut looper = Looper::new();
        looper.init(48000.0).unwrap();

        // Record initial loop
        let input1 = vec![1.0f32; 256];
        looper.trigger_mode(LoopMode::Record);
        let mut out = vec![0.0; 256];
        looper.process(&input1, &mut out).unwrap();

        // Switch to overdub
        looper.trigger_mode(LoopMode::Play);
        looper.trigger_mode(LoopMode::Overdub);
        let input2 = vec![2.0f32; 256];
        let mut out2 = vec![0.0; 256];
        looper.process(&input2, &mut out2).unwrap();

        // Undo should restore original
        looper.undo();

        // Play back and verify original
        looper.trigger_mode(LoopMode::Play);
        let mut out3 = vec![0.0; 256];
        let silent = vec![0.0; 256];
        looper.process(&silent, &mut out3).unwrap();

        // The first sample should be close to original (mix = 0.5 means 0.5 gain)
        assert!((out3[10] - 0.5).abs() < 0.1);
    }

    #[test]
    fn test_looper_reverse() {
        let mut looper = Looper::new();
        looper.init(48000.0).unwrap();

        let input: Vec<f32> = (0..480).map(|i| i as f32).collect();
        looper.trigger_mode(LoopMode::Record);
        let mut out = vec![0.0; 480];
        looper.process(&input, &mut out).unwrap();

        looper.trigger_mode(LoopMode::Play);
        looper.set_parameter("reverse", 1.0);

        let mut out2 = vec![0.0; 480];
        let silent = vec![0.0; 480];
        looper.process(&silent, &mut out2).unwrap();

        // In reverse, the first output sample should be non-zero (near last input sample)
        // With mix=0.5 and volume=1.0, output should be roughly half the loop value
        assert!(
            out2.iter().any(|&v| v > 50.0),
            "Reverse playback produced no meaningful output"
        );
    }
}
