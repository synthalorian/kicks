// Kicks — Direct Convolution Engine
//
// Real-time convolution using direct convolution with an impulse response.
// Efficient enough for guitar cab IRs (typically 512–2048 samples at 48 kHz).
// For longer IRs (>4096 samples), swap in a partitioned FFT convolver later.

/// Maximum supported IR length in samples (~170 ms at 48 kHz).
const MAX_IR_LENGTH: usize = 8192;

/// A direct-form convolver for real-time audio processing.
///
/// Maintains a delay line of recent input samples and computes the
/// convolution sum directly. Zero-latency (no FFT blocks needed).
pub struct Convolver {
    /// The impulse response (time domain, length = ir_len).
    ir: Vec<f32>,
    /// Length of the impulse response in samples.
    ir_len: usize,
    /// Delay line holding the last `ir_len` input samples (ring buffer).
    delay: Vec<f32>,
    /// Write position in the delay line ring buffer.
    write_pos: usize,
    /// Sample rate of the IR (for metadata display).
    sample_rate: f32,
}

impl Convolver {
    /// Create a new convolver from impulse response data.
    ///
    /// `ir_data` — raw float samples of the impulse response.
    /// For multi-channel IRs, only the first channel should be passed
    /// (down-mixed to mono by the caller).
    pub fn new(ir_data: &[f32], sample_rate: f32) -> Self {
        let ir_len = ir_data.len().min(MAX_IR_LENGTH);
        let mut ir = vec![0.0f32; ir_len];
        ir.copy_from_slice(&ir_data[..ir_len]);

        Self {
            ir_len,
            ir,
            delay: vec![0.0f32; ir_len.max(1)],
            write_pos: 0,
            sample_rate,
        }
    }

    /// Process a block of audio samples through the convolver.
    ///
    /// `input` and `output` must be the same length.
    /// Implements: y[n] = sum_{k=0}^{ir_len-1} ir[k] * x[n - k]
    pub fn process(&mut self, input: &[f32], output: &mut [f32]) {
        let n_samples = input.len();
        let ir_len = self.ir_len;

        if ir_len == 0 {
            output.copy_from_slice(input);
            return;
        }

        for i in 0..n_samples {
            // Write the current input sample into the delay line
            self.delay[self.write_pos] = input[i];

            // Compute convolution: y = sum(ir[k] * delay[write_pos - k])
            // Ring buffer: delay[(write_pos - k) mod ir_len]
            let mut sum = 0.0f32;
            let mut read_pos = self.write_pos;

            for k in 0..ir_len {
                sum += self.ir[k] * self.delay[read_pos];
                // Move backwards in the ring buffer
                read_pos = if read_pos == 0 { ir_len - 1 } else { read_pos - 1 };
            }

            output[i] = sum;

            // Advance write position
            self.write_pos = (self.write_pos + 1) % ir_len;
        }
    }

    /// Process with a wet/dry mix.
    pub fn process_mixed(&mut self, input: &[f32], output: &mut [f32], wet: f32, dry: f32) {
        let n_samples = input.len();
        let ir_len = self.ir_len;

        if ir_len == 0 {
            output.copy_from_slice(input);
            return;
        }

        for i in 0..n_samples {
            self.delay[self.write_pos] = input[i];

            let mut sum = 0.0f32;
            let mut read_pos = self.write_pos;

            for k in 0..ir_len {
                sum += self.ir[k] * self.delay[read_pos];
                read_pos = if read_pos == 0 { ir_len - 1 } else { read_pos - 1 };
            }

            output[i] = input[i] * dry + sum * wet;
            self.write_pos = (self.write_pos + 1) % ir_len;
        }
    }

    /// Normalize the impulse response so its peak amplitude is 1.0.
    pub fn normalize(&mut self) {
        let peak = self.ir.iter().cloned().fold(0.0f32, f32::max).abs();
        if peak > 0.0 {
            let inv = 1.0 / peak;
            for sample in self.ir.iter_mut() {
                *sample *= inv;
            }
        }
    }

    /// Trim trailing silence from the IR.
    pub fn trim(&mut self, threshold: f32) {
        // Find the last sample above threshold
        let mut last_nonzero = 0;
        for (i, &sample) in self.ir.iter().enumerate().rev() {
            if sample.abs() > threshold {
                last_nonzero = i + 1;
                break;
            }
        }
        if last_nonzero > 0 && last_nonzero < self.ir_len {
            self.ir.truncate(last_nonzero);
            self.ir_len = last_nonzero;
            self.delay.resize(last_nonzero.max(1), 0.0);
            self.write_pos = 0;
        }
    }

    // ── Accessors ──────────────────────────────────────────────────────────────

    /// Length of the impulse response in samples.
    pub fn ir_length(&self) -> usize {
        self.ir_len
    }

    /// Length of the impulse response in milliseconds.
    pub fn ir_length_ms(&self) -> f32 {
        if self.sample_rate <= 0.0 {
            return 0.0;
        }
        (self.ir_len as f32) / self.sample_rate * 1000.0
    }

    /// Sample rate of the impulse response.
    pub fn sample_rate(&self) -> f32 {
        self.sample_rate
    }

    /// Whether an IR is loaded.
    pub fn is_loaded(&self) -> bool {
        self.ir_len > 0
    }

    /// Reset the delay line to zero (clears tail).
    pub fn reset(&mut self) {
        self.delay.fill(0.0);
        self.write_pos = 0;
    }
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convolver_identity() {
        // IR = [1.0] → convolution is identity
        let ir = vec![1.0f32];
        let mut conv = Convolver::new(&ir, 48000.0);

        let input: Vec<f32> = (0..64).map(|i| (i as f32 / 64.0) * 0.5 - 0.25).collect();
        let mut output = vec![0.0f32; 64];
        conv.process(&input, &mut output);

        for (out, inp) in output.iter().zip(input.iter()) {
            assert!((out - inp).abs() < 1e-6,
                "Mismatch: expected {}, got {}", inp, out);
        }
    }

    #[test]
    fn test_convolver_known_ir() {
        // IR = [1.0, 0.5] → y[n] = x[n] + 0.5 * x[n-1]
        let ir = vec![1.0, 0.5];
        let mut conv = Convolver::new(&ir, 48000.0);

        let input = vec![1.0, 0.0, 0.0, 0.0];
        let mut output = vec![0.0f32; 4];
        conv.process(&input, &mut output);

        // y[0] = 1.0*1.0 + 0.5*0.0 = 1.0
        // y[1] = 1.0*0.0 + 0.5*1.0 = 0.5
        // y[2] = 1.0*0.0 + 0.5*0.0 = 0.0
        // y[3] = 1.0*0.0 + 0.5*0.0 = 0.0
        assert!((output[0] - 1.0).abs() < 1e-6);
        assert!((output[1] - 0.5).abs() < 1e-6);
        assert!((output[2] - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_convolver_empty_ir() {
        let ir: Vec<f32> = vec![];
        let mut conv = Convolver::new(&ir, 48000.0);
        let input = vec![0.5f32; 32];
        let mut output = vec![1.0f32; 32];
        conv.process(&input, &mut output);
        // Empty IR = passthrough
        for i in 0..32 {
            assert!((output[i] - input[i]).abs() < 1e-6);
        }
    }

    #[test]
    fn test_convolver_mixed() {
        let ir = vec![0.5f32];
        let mut conv = Convolver::new(&ir, 48000.0);
        let input = vec![1.0f32; 16];
        let mut output = vec![0.0f32; 16];
        // 50% wet, 50% dry
        conv.process_mixed(&input, &mut output, 0.5, 0.5);
        // y[n] = 0.5*x[n] + 0.5*(0.5*x[n]) = 0.75*x[n]
        for out in output.iter().take(16) {
            assert!((out - 0.75).abs() < 1e-6);
        }
    }

    #[test]
    fn test_convolver_normalize() {
        let ir = vec![0.0, 0.5, 0.0, 2.0, 0.0];
        let mut conv = Convolver::new(&ir, 48000.0);
        conv.normalize();
        let peak = conv.ir.iter().cloned().fold(0.0f32, f32::max);
        assert!((peak - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_convolver_trim() {
        let ir = vec![0.5, 0.3, 0.1, 0.01, 0.001, 0.0, 0.0];
        let mut conv = Convolver::new(&ir, 48000.0);
        conv.trim(0.01);
        // Samples > 0.01: 0.5, 0.3, 0.1 → last_nonzero = index 3 → 3 samples
        assert_eq!(conv.ir_length(), 3);
        assert!((conv.ir[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_convolver_reset() {
        let ir = vec![0.5f32; 32];
        let mut conv = Convolver::new(&ir, 48000.0);
        let input = vec![1.0f32; 64];
        let mut output1 = vec![0.0f32; 64];
        conv.process(&input, &mut output1);

        conv.reset();

        let mut output2 = vec![0.0f32; 64];
        conv.process(&input, &mut output2);
        // After reset, the delay line is cleared, so the result should be
        // the same as the first run (which started from zeros)
        for i in 0..64 {
            assert!((output2[i] - output1[i]).abs() < 1e-6);
        }
    }

    #[test]
    fn test_convolver_metadata() {
        let ir = vec![1.0f32; 1024];
        let conv = Convolver::new(&ir, 44100.0);
        assert_eq!(conv.ir_length(), 1024);
        assert!((conv.sample_rate() - 44100.0).abs() < 1.0);
        assert!(conv.is_loaded());
    }
}
