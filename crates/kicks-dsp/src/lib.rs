// Kicks — Digital Signal Processing Engine
//
// Real-time audio processing pipeline for the Kicks guitar workstation.
// Provides Boost, Amp, Cab IR, Delay, and Reverb processing stages
// with CPAL (default) or JACK audio I/O.

pub mod convolution;
pub mod engine;
pub mod plugins;
pub mod audio_io;
#[cfg(feature = "cpal-backend")]
pub mod param;

pub use audio_io::{AudioConfig, CpalAudioIO, DeviceInfo, JackAudioIO, JackConfig, list_audio_devices};
pub use convolution::Convolver;
pub use engine::{AudioEngine, KicksEngine};
pub use plugins::{Cab, compute_rms, IrInfo, Plugin, PluginRegistry};
