// Kicks — Metronome Plugin
//
// A practice metronome that generates click sounds at a configurable BPM.
// Produces a sharp "tick" on the downbeat and a softer "tock" on other beats.
// Supports variable time signatures (beats per bar).

use crate::plugins::Plugin;

/// Samples for a single metronome click (pre-rendered impulse + decay).
const CLICK_DURATION_MS: f32 = 30.0;

pub struct Metronome {
    /// BPM (20..300)
    bpm: f32,
    /// Beats per bar (1..16)
    beats_per_bar: u8,
    /// Current beat within the bar (0-based).
    current_beat: u8,
    /// Sample rate.
    sample_rate: f32,
    /// Samples per beat.
    samples_per_beat: f32,
    /// Sample counter within current beat.
    sample_counter: f32,
    /// Output volume (0..1).
    volume: f32,
    /// Whether the metronome is running.
    running: bool,
    /// Downbeat click buffer (pre-rendered).
    downbeat_click: Vec<f32>,
    /// Regular beat click buffer (pre-rendered).
    regular_click: Vec<f32>,
    /// Position within current click playback.
    click_position: usize,
    /// Whether we're currently playing a click.
    playing_click: bool,
    /// Whether the current click is a downbeat.
    is_downbeat: bool,
    /// Click blend (0 = no click, 1 = full click volume mixed with guitar).
    click_blend: f32,
}

impl Default for Metronome {
    fn default() -> Self {
        Self::new()
    }
}

impl Metronome {
    pub fn new() -> Self {
        Self {
            bpm: 120.0,
            beats_per_bar: 4,
            current_beat: 0,
            sample_rate: 48000.0,
            samples_per_beat: 24000.0, // 120 BPM at 48kHz
            sample_counter: 0.0,
            volume: 0.5,
            running: false,
            downbeat_click: Vec::new(),
            regular_click: Vec::new(),
            click_position: 0,
            playing_click: false,
            is_downbeat: false,
            click_blend: 0.5,
        }
    }

    fn update_timing(&mut self) {
        self.samples_per_beat = (60.0 / self.bpm) * self.sample_rate;
    }

    /// Render a click sound: a short high-frequency impulse with exponential decay.
    fn render_click(sample_rate: f32, freq: f32, amp: f32, duration_ms: f32) -> Vec<f32> {
        let num_samples = ((duration_ms / 1000.0) * sample_rate).ceil() as usize;
        let mut click = Vec::with_capacity(num_samples);
        let decay = -5.0 / sample_rate; // exponential decay rate
        for i in 0..num_samples {
            let t = i as f32 / sample_rate;
            let envelope = (decay * i as f32).exp();
            // Mix of fundamental and higher harmonics for a "clicky" sound
            let sample = amp * envelope * (
                (2.0 * std::f32::consts::PI * freq * t).sin() * 0.7 +
                (2.0 * std::f32::consts::PI * freq * 2.5 * t).sin() * 0.3
            );
            click.push(sample);
        }
        click
    }

    fn init_clicks(&mut self) {
        // Downbeat: higher freq, louder
        self.downbeat_click = Self::render_click(self.sample_rate, 2000.0, 0.4, CLICK_DURATION_MS);
        // Regular beat: slightly lower freq, softer
        self.regular_click = Self::render_click(self.sample_rate, 1500.0, 0.25, CLICK_DURATION_MS);
    }

    fn trigger_click(&mut self, is_downbeat: bool) {
        self.playing_click = true;
        self.click_position = 0;
        self.is_downbeat = is_downbeat;
    }
}

impl Plugin for Metronome {
    fn name(&self) -> &str {
        "metronome"
    }

    fn init(&mut self, sample_rate: f64) -> anyhow::Result<()> {
        self.sample_rate = sample_rate as f32;
        self.update_timing();
        self.init_clicks();
        Ok(())
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()> {
        if !self.running {
            output.copy_from_slice(input);
            return Ok(());
        }

        let vol = self.volume;
        let click_blend = self.click_blend;

        for (i, &input_sample) in input.iter().enumerate() {
            let mut out = input_sample; // Guitar always passes through

            // Check if we've reached the next beat
            if self.sample_counter >= self.samples_per_beat {
                self.sample_counter -= self.samples_per_beat;
                let is_downbeat = self.current_beat == 0;
                self.trigger_click(is_downbeat);
                self.current_beat = (self.current_beat + 1) % self.beats_per_bar;
            }

            // Mix in click on top of guitar signal
            if self.playing_click {
                let click_buf = if self.is_downbeat {
                    &self.downbeat_click
                } else {
                    &self.regular_click
                };

                if self.click_position < click_buf.len() {
                    out += click_buf[self.click_position] * vol * click_blend;
                    self.click_position += 1;
                } else {
                    self.playing_click = false;
                }
            }

            output[i] = out;
            self.sample_counter += 1.0;
        }
        Ok(())
    }

    fn get_parameter(&self, id: &str) -> Option<f32> {
        match id {
            "bpm" => Some(self.bpm), // Raw BPM (20..300)
            "beats_per_bar" => Some(self.beats_per_bar as f32),
            "volume" => Some(self.volume),
            "running" => Some(if self.running { 1.0 } else { 0.0 }),
            "click_blend" => Some(self.click_blend),
            _ => None,
        }
    }

    fn set_parameter(&mut self, id: &str, value: f32) {
        match id {
            "bpm" => {
                self.bpm = value.clamp(20.0, 300.0);
                self.update_timing();
            }
            "beats_per_bar" => {
                self.beats_per_bar = (value.clamp(1.0, 16.0) as u8).max(1);
            }
            "volume" => self.volume = value.clamp(0.0, 1.0),
            "running" => self.running = value > 0.5,
            "click_blend" => self.click_blend = value.clamp(0.0, 1.0),
            _ => {}
        }
    }

    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metronome_generates_clicks() {
        let mut metro = Metronome::new();
        metro.init(48000.0).unwrap();
        metro.set_parameter("running", 1.0);
        metro.set_parameter("bpm", 120.0); // 120 BPM

        // At 120 BPM, 48kHz = 24000 samples per beat
        let mut output = vec![0.0f32; 24000 * 4]; // 4 beats worth
        let input = vec![0.0f32; 24000 * 4];
        metro.process(&input, &mut output).unwrap();

        // Output should have non-zero samples (the clicks)
        assert!(output.iter().any(|&x| x != 0.0));
    }

    #[test]
    fn test_metronome_bypass_when_stopped() {
        let mut metro = Metronome::new();
        metro.init(48000.0).unwrap();
        let input = vec![0.5f32; 256];
        let mut output = vec![0.0; 256];
        metro.process(&input, &mut output).unwrap();
        assert_eq!(input, output);
    }
}
