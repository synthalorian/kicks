// Kicks — Digital Signal Processing Engine
//
// Real-time audio processing pipeline for the Kicks guitar workstation.
// Provides Boost, Amp, Cab IR, Delay, Reverb, Tuner, Metronome, Looper
// and BassAmp processing stages with CPAL (default) or JACK audio I/O.

pub mod convolution;
pub mod engine;
pub mod nam;
pub mod plugins;
pub mod audio_io;
pub mod bass_amp;
pub mod tuner;
pub mod metronome;
pub mod looper;
#[cfg(feature = "fft-convolution")]
pub mod fft_convolution;
#[cfg(feature = "cpal-backend")]
pub mod param;

pub use audio_io::{AudioConfig, CpalAudioIO, DeviceInfo, JackAudioIO, JackConfig, list_audio_devices};
pub use convolution::Convolver;
pub use engine::{AudioEngine, KicksEngine};
pub use nam::{NamModelInfo, NeuralModel};
pub use plugins::{Cab, compute_rms, IrInfo, Nam, Plugin, PluginRegistry};
pub use bass_amp::BassAmp;
pub use tuner::Tuner;
pub use metronome::Metronome;
pub use looper::Looper;
#[cfg(feature = "fft-convolution")]
pub use fft_convolution::FftConvolver;
