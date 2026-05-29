// Kicks — Audio I/O Backend
//
// Abstracts real-time audio I/O behind an optional CPAL (default) or JACK backend.
// CPAL gives us cross-platform audio (ALSA/PipeWire on Linux, CoreAudio on macOS,
// WASAPI on Windows). JACK remains as a feature-gated alternative.
//
// Real-time safety:
//   - Parameter changes are sent via a lock-free SPSC ring buffer so the main
//     thread never holds the engine mutex for `set_parameter`.
//   - The audio callback still uses `try_lock` on the engine, but contention
//     is now extremely rare (only non-parameter ops like IR loading or engine
//     start/stop hold the lock).

use std::sync::{Arc, Mutex};

use anyhow::{Context, Result};

use crate::engine::{AudioEngine, KicksEngine};

// ═══════════════════════════════════════════════════════════════════════════════
// AudioConfig
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for audio I/O.
#[derive(Debug, Clone)]
pub struct AudioConfig {
    /// Sample rate in Hz (default: 48000).
    pub sample_rate: f64,
    /// Buffer size in frames (default: 256).
    pub buffer_size: u32,
    /// Optional output device name. If None, uses default.
    pub output_device: Option<String>,
    /// Optional input device name. If None, uses default.
    pub input_device: Option<String>,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 48000.0,
            buffer_size: 256,
            output_device: None,
            input_device: None,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// CpalAudioIO — Cross-platform audio via CPAL
// ═══════════════════════════════════════════════════════════════════════════════

/// CPAL-based audio I/O backend.
///
/// Opens full-duplex input+output streams on the calling thread.
/// The engine is shared via `Arc<Mutex<KicksEngine>>` so the main thread
/// can control the engine while audio callbacks process in real-time.
///
/// Parameter changes are lock-free: `start()` accepts the consumer end of an
/// SPSC channel (`ParamConsumer`), which the output callback drains before
/// each `process_all` call.  The producer lives in `AppState` so the main
/// thread never needs to lock the engine mutex for parameter changes.
///
/// Audio flow: Input callback → ring buffer → Output callback → KicksEngine → speakers
///
/// Requires CPAL 0.17+ (Stream is Send+Sync across all platforms).
#[derive(Default)]
pub struct CpalAudioIO {
    #[cfg(feature = "cpal-backend")]
    _input_stream: Option<cpal::Stream>,
    #[cfg(feature = "cpal-backend")]
    _output_stream: Option<cpal::Stream>,
    config: AudioConfig,
    active: bool,
}

impl CpalAudioIO {
    pub fn new() -> Self {
        Self::default()
    }

    /// Start full-duplex audio I/O with the given engine and config.
    ///
    /// Opens input+output CPAL streams on the current default devices.
    ///
    /// `param_rx` is the consumer end of a lock-free SPSC channel. The output
    /// callback drains it before each `process_all` cycle, applying pending
    /// parameter changes to the engine.  Because the main thread pushes to
    /// the producer side (never locking the engine mutex), `try_lock` in the
    /// callback almost always succeeds.
    pub fn start(
        &mut self,
        engine: Arc<Mutex<KicksEngine>>,
        config: AudioConfig,
        #[cfg(feature = "cpal-backend")] param_rx: ringbuf::HeapCons<(String, f32)>,
    ) -> Result<()> {
        #[cfg(feature = "cpal-backend")]
        {
            self.stop();
            self.config = config.clone();

            use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
            use ringbuf::traits::{Consumer, Producer, Split};
            use ringbuf::HeapRb;

            let host = cpal::default_host();

            let output_device = config
                .output_device
                .as_ref()
                .and_then(|name| {
                    #[allow(deprecated)]
                    host.output_devices()
                        .ok()?
                        .find(|d| d.name().map(|n| n == *name).unwrap_or(false))
                })
                .or_else(|| host.default_output_device())
                .context("No audio output device found")?;

            let input_device = config
                .input_device
                .as_ref()
                .and_then(|name| {
                    #[allow(deprecated)]
                    host.input_devices()
                        .ok()?
                        .find(|d| d.name().map(|n| n == *name).unwrap_or(false))
                })
                .or_else(|| host.default_input_device());

            let sr = config.sample_rate as u32;
            let bs = config.buffer_size;

            let stream_config = cpal::StreamConfig {
                channels: 2,
                sample_rate: sr,
                buffer_size: cpal::BufferSize::Fixed(bs),
            };

            let err_fn = |err| tracing::error!("CPAL stream error: {}", err);

            // Ring buffer: input callback → output callback (mono)
            let ring_capacity = bs as usize * 8;
            let ring = HeapRb::<f32>::new(ring_capacity);
            let (producer, mut consumer) = ring.split();

            // ── Input stream (optional — only if device exists) ──
            let input_stream = input_device.and_then(|dev| {
                let mut prod = producer;
                dev.build_input_stream::<f32, _, _>(
                    &stream_config,
                    move |data: &[f32], _: &cpal::InputCallbackInfo| {
                        // CPAL gives us interleaved stereo; convert to mono by taking left channel
                        for frame in data.chunks(2) {
                            let mono = frame.first().copied().unwrap_or(0.0);
                            let _ = prod.try_push(mono);
                        }
                    },
                    err_fn,
                    None,
                )
                .ok()
            });

            if let Some(ref stream) = input_stream {
                if let Err(e) = stream.play() {
                    tracing::warn!("Failed to start input stream: {}", e);
                }
            }

            // ── Output stream ──
            // Pre-allocated input buffer avoids allocation in audio callback
            let mut input_buf = vec![0.0f32; bs as usize];
            let mut output_buf = vec![0.0f32; bs as usize];
            let eng = engine;
            let mut param_rx = param_rx;

            let output_stream = output_device
                .build_output_stream::<f32, _, _>(
                    &stream_config,
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        // Drain ring buffer into mono input_buf
                        for input_slot in input_buf.iter_mut() {
                            *input_slot = consumer.try_pop().unwrap_or(0.0);
                        }

                        if let Ok(mut eng) = eng.try_lock() {
                            // Drain pending parameter changes (lock-free SPSC
                            // queue) before processing audio.  The main thread
                            // pushes to the producer side without holding the
                            // engine mutex, so `try_lock` almost always succeeds.
                            while let Some((id, value)) = param_rx.try_pop() {
                                eng.set_parameter(&id, value);
                            }

                            let _ = eng.process(&input_buf, &mut output_buf);
                        }
                        // If try_lock fails, output_buf stays zero (silence)

        // Duplicate mono output to stereo interleaved
                        for (frame, &mono) in data.chunks_mut(2).zip(output_buf.iter()) {
                            frame[0] = mono;
                            if let Some(right) = frame.get_mut(1) {
                                *right = mono;
                            }
                        }
                    },
                    |err| tracing::error!("CPAL output stream error: {}", err),
                    None,
                )
                .context("Failed to build output audio stream")?;

            output_stream
                .play()
                .context("Failed to start output audio stream")?;

            self._input_stream = input_stream;
            self._output_stream = Some(output_stream);
            self.active = true;

            tracing::info!(
                "CPAL audio started: {} Hz, buffer {} frames, stereo",
                sr,
                bs
            );
        }

        #[cfg(not(feature = "cpal-backend"))]
        {
            let _ = (engine, config);
            tracing::warn!("CPAL backend not compiled in; audio I/O disabled");
        }

        Ok(())
    }

    /// Stop audio I/O by dropping stream handles.
    pub fn stop(&mut self) {
        #[cfg(feature = "cpal-backend")]
        {
            self._output_stream = None;
            self._input_stream = None;
            tracing::info!("CPAL audio stopped");
        }
        self.active = false;
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn config(&self) -> &AudioConfig {
        &self.config
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Device enumeration
// ═══════════════════════════════════════════════════════════════════════════════

/// Information about a single audio device discovered on the system.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DeviceInfo {
    pub name: String,
    pub is_input: bool,
    pub is_output: bool,
}

/// Enumerate available audio devices via CPAL.
///
/// Returns a list of `DeviceInfo` entries.  When the `cpal-backend` feature
/// is disabled, returns an empty vec.
pub fn list_audio_devices() -> Vec<DeviceInfo> {
    #[cfg(feature = "cpal-backend")]
    {
        use cpal::traits::{DeviceTrait, HostTrait};
        let host = cpal::default_host();
        let mut devices = Vec::new();
        if let Ok(devs) = host.devices() {
            for dev in devs {
                let name = dev.description()
                    .map(|d| d.to_string())
                    .unwrap_or_else(|_| String::new());
                let is_input = dev.supported_input_configs().map(|c| c.count() > 0).unwrap_or(false);
                let is_output = dev.supported_output_configs().map(|c| c.count() > 0).unwrap_or(false);
                devices.push(DeviceInfo { name, is_input, is_output });
            }
        }
        devices
    }
    #[cfg(not(feature = "cpal-backend"))]
    {
        vec![]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// JackAudioIO — JACK backend (feature-gated alternative)
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for JACK audio I/O.
#[derive(Debug, Clone)]
pub struct JackConfig {
    pub client_name: String,
    pub input_ports: Vec<String>,
    pub output_ports: Vec<String>,
}

/// A JACK audio I/O client.
/// When the `jack-backend` feature is enabled, this provides a real JACK client.
/// Otherwise, it acts as a no-op stub.
pub struct JackAudioIO {
    #[cfg(feature = "jack-backend")]
    client: Option<jack::Client>,
    #[cfg(feature = "jack-backend")]
    input_port: Option<jack::Port<jack::AudioIn>>,
    #[cfg(feature = "jack-backend")]
    output_port: Option<jack::Port<jack::AudioOut>>,

    config: JackConfig,
    active: bool,
}

impl JackAudioIO {
    pub fn new(config: JackConfig) -> Self {
        Self {
            #[cfg(feature = "jack-backend")]
            client: None,
            #[cfg(feature = "jack-backend")]
            input_port: None,
            #[cfg(feature = "jack-backend")]
            output_port: None,
            config,
            active: false,
        }
    }

    #[must_use]
    pub fn with_config(config: JackConfig) -> Self {
        Self::new(config)
    }

    /// Open JACK client and register ports.
    pub fn open(&mut self) -> Result<()> {
        #[cfg(feature = "jack-backend")]
        {
            let (client, _) = jack::Client::new(
                &self.config.client_name,
                jack::ClientOptions::NO_START_SERVER,
            )?;

            let input = client.register_port("input", jack::AudioIn)?;
            let output = client.register_port("output", jack::AudioOut)?;

            self.client = Some(client);
            self.input_port = Some(input);
            self.output_port = Some(output);
            self.active = true;

            tracing::info!(
                "JACK client '{}' opened successfully",
                self.config.client_name
            );
        }

        #[cfg(not(feature = "jack-backend"))]
        {
            tracing::warn!("JACK backend not compiled in; audio I/O disabled");
            self.active = false;
        }

        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    /// Close the JACK client.
    pub fn close(&mut self) -> Result<()> {
        #[cfg(feature = "jack-backend")]
        if let Some(client) = self.client.take() {
            drop(client);
            tracing::info!("JACK client closed");
        }
        self.active = false;
        Ok(())
    }
}
