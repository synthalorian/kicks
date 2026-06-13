use std::path::Path;
use std::sync::{Arc, Mutex};

use kicks_core::config::{AudioBackend, EngineMode};
use kicks_core::signal_chain::PluginType;
use kicks_dsp::param::param_channel;
use kicks_dsp::{AudioConfig, AudioEngine, CpalAudioIO, JackAudioIO, KicksEngine};
use serde::Serialize;
use tauri::State;

use crate::AppState;

/// Status information about the audio engine.
#[derive(Debug, Serialize)]
pub struct EngineStatus {
    pub running: bool,
    pub sample_rate: f64,
    pub buffer_size: u32,
    pub backend: String,
    pub mode: String,
}

/// Helper to read a WAV file and load it into the engine's Cab plugin.
fn load_ir_from_file(path: &str, eng: &mut KicksEngine) -> Result<super::ir::IrLoadResult, String> {
    let fpath = Path::new(path);
    let mut reader =
        hound::WavReader::open(fpath).map_err(|e| format!("Failed to open WAV: {}", e))?;
    let spec = reader.spec();
    let channels = spec.channels as usize;

    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().filter_map(|s| s.ok()).collect(),
        hound::SampleFormat::Int => match spec.bits_per_sample {
            16 => reader
                .samples::<i16>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / 32768.0)
                .collect(),
            24 => reader
                .samples::<i32>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / 8388608.0)
                .collect(),
            32 => reader
                .samples::<i32>()
                .filter_map(|s| s.ok())
                .map(|s| s as f32 / 2147483648.0)
                .collect(),
            _ => return Err(format!("Unsupported bit depth: {}", spec.bits_per_sample)),
        },
    };

    let mono_samples: Vec<f32> = if channels > 1 {
        let frame_count = samples.len() / channels;
        (0..frame_count)
            .map(|f| {
                (0..channels)
                    .map(|ch| samples[f * channels + ch])
                    .sum::<f32>()
                    / channels as f32
            })
            .collect()
    } else {
        samples
    };

    let file_name = fpath
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    eng.load_ir_to_cab(
        path.to_string(),
        mono_samples.clone(),
        spec.sample_rate as f32,
    );

    let total_samples = mono_samples.len();
    let ir_len_ms = if spec.sample_rate > 0 {
        (total_samples as f32) / spec.sample_rate as f32 * 1000.0
    } else {
        0.0
    };

    Ok(super::ir::IrLoadResult {
        path: path.to_string(),
        file_name,
        sample_rate: spec.sample_rate,
        length_samples: total_samples,
        length_ms: ir_len_ms as u32,
    })
}

/// Start the audio engine. Depending on `config.active_engine`, this either
/// launches the internal DSP engine or connects to (and optionally launches)
/// a headless Guitarix process via JSON-RPC.
#[tauri::command]
pub async fn start_engine(state: State<'_, AppState>) -> Result<(), String> {
    let (engine_mode, sample_rate, buffer_size, input_device, output_device, audio_backend, jack_client_name, guitarix_host, guitarix_port) = {
        let config = state.config.lock().map_err(|e| e.to_string())?;
        (
            config.active_engine.clone(),
            config.sample_rate as f64,
            config.buffer_size,
            config.input_device.clone(),
            config.output_device.clone(),
            config.audio_backend.clone(),
            config.jack_client_name.clone(),
            config.guitarix_host.clone(),
            config.guitarix_port,
        )
    };

    match engine_mode {
        EngineMode::Guitarix => start_guitarix_mode(&state, &guitarix_host, guitarix_port).await,
        EngineMode::Auto => {
            // Try Guitarix first; fall back to internal on failure
            if let Err(e) = start_guitarix_mode(&state, &guitarix_host, guitarix_port).await {
                tracing::warn!("Guitarix mode failed ({}), falling back to internal", e);
                start_internal_mode(
                    &state,
                    sample_rate,
                    buffer_size,
                    &input_device,
                    &output_device,
                    audio_backend,
                    &jack_client_name,
                )
                .await
                } else {
                Ok(())
                }
                }
                EngineMode::Internal => {
                start_internal_mode(
                &state,
                sample_rate,
                buffer_size,
                &input_device,
                &output_device,
                audio_backend,
                &jack_client_name,
                )
                .await
                }
    }
}

/// Launch/connect to a headless Guitarix process via JSON-RPC.
async fn start_guitarix_mode(
    state: &State<'_, AppState>,
    host: &str,
    port: u16,
) -> Result<(), String> {
    // If a guitarix process is already managed, reuse it
    let needs_launch = {
        let mut proc_guard = state.guitarix_process.lock().map_err(|e| e.to_string())?;
        proc_guard.as_mut().map(|p| !p.is_running()).unwrap_or(true)
    };

    if needs_launch {
        match guitarix_rpc::GuitarixProcess::launch(port) {
            Ok(proc) => {
                tracing::info!("Launched guitarix headless on port {}", port);
                *state.guitarix_process.lock().map_err(|e| e.to_string())? = Some(proc);
                // Give guitarix a moment to bind its RPC port
                tokio::time::sleep(std::time::Duration::from_millis(800)).await;
            }
            Err(e) => {
                tracing::warn!("Failed to launch guitarix: {}. Trying existing instance...", e);
            }
        }
    }

    // Connect via JSON-RPC
    let client = guitarix_rpc::GuitarixClient::connect(host, port)
        .await
        .map_err(|e| format!("Failed to connect to guitarix at {}:{}: {}", host, port, e))?;

    // Verify connection by fetching version
    let mut client = client;
    let version = client
        .get_version()
        .await
        .map_err(|e| format!("Guitarix RPC handshake failed: {}", e))?;
    tracing::info!("Connected to guitarix version {}", version);

    // Store client and mark mode
    *state.guitarix_client.lock().map_err(|e| e.to_string())? = Some(client);
    *state.active_mode.lock().map_err(|e| e.to_string())? = "guitarix".to_string();

    tracing::info!("Guitarix engine started (RPC mode)");
    Ok(())
}

/// Start the internal DSP engine with the selected audio backend.
#[allow(clippy::too_many_arguments)]
async fn start_internal_mode(
    state: &State<'_, AppState>,
    sample_rate: f64,
    buffer_size: u32,
    input_device: &str,
    output_device: &str,
    audio_backend: AudioBackend,
    jack_client_name: &str,
) -> Result<(), String> {
    let mut engine_guard = state.engine.lock().map_err(|e| e.to_string())?;

    if engine_guard.is_some() {
        tracing::info!("Internal engine already running; skipping start");
        return Ok(());
    }

    let eng = Arc::new(Mutex::new(KicksEngine::new()));
    {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        eng_inner
            .init(sample_rate, buffer_size)
            .map_err(|e| e.to_string())?;
    }

    // Auto-load any previously saved IR
    let config_ir = state.config.lock().map_err(|e| e.to_string())?;
    if !config_ir.active_ir_path.is_empty() {
        let ir_path = config_ir.active_ir_path.clone();
        drop(config_ir);

        let fpath = Path::new(&ir_path);
        if fpath.exists() {
            let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
            if let Ok(ir_result) = load_ir_from_file(&ir_path, &mut eng_inner) {
                tracing::info!(
                    "Auto-loaded IR: {} ({} samples)",
                    ir_result.file_name,
                    ir_result.length_samples
                );
            } else {
                tracing::warn!("Failed to auto-load IR from saved path: {}", ir_path);
            }
        } else {
            tracing::warn!("Saved IR path no longer exists: {}", ir_path);
        }
    } else {
        drop(config_ir);
    }

    // If the signal chain has a BassAmp, set bass_mode on the engine
    let chain = state.signal_chain.lock().map_err(|e| e.to_string())?;
    {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        if let Some(slot) = chain
            .slots
            .iter()
            .find(|s| matches!(s.plugin_type, PluginType::BassAmp))
        {
            let bass_val = slot.parameters.get("bass_mode").copied().unwrap_or(1.0);
            eng_inner.set_parameter("bass_mode", bass_val);
        }
        // Apply all saved signal chain parameters to the engine
        for slot in &chain.slots {
            let plugin_name = slot.id.clone();
            eng_inner.set_plugin_enabled(&plugin_name, slot.enabled);
            for (param_id, value) in &slot.parameters {
                eng_inner.set_parameter_on_plugin(&plugin_name, param_id, *value);
            }
        }
    }
    drop(chain);

    let eng_for_io = eng.clone();
    *engine_guard = Some(eng);
    drop(engine_guard);

    match audio_backend {
        AudioBackend::Jack => {
            let mut jack_io = state.jack_audio_io.lock().map_err(|e| e.to_string())?;
            if jack_io.is_none() {
                *jack_io = Some(JackAudioIO::new(kicks_dsp::JackConfig {
                    client_name: if jack_client_name.is_empty() {
                        "Kicks".to_string()
                    } else {
                        jack_client_name.to_string()
                    },
                }));
            }
            if let Some(ref mut io) = *jack_io {
                let cpu_load = Arc::clone(&state.cpu_load);
                io.start(eng_for_io, Some(cpu_load))
                    .map_err(|e| format!("Failed to start JACK audio: {}", e))?;
            }
            tracing::info!("Internal engine started with JACK I/O");
        }
        AudioBackend::Cpal => {
            let (param_tx, param_rx) = param_channel();
            *state.param_tx.lock().map_err(|e| e.to_string())? = Some(param_tx);

            let audio_config = AudioConfig {
                sample_rate,
                buffer_size,
                output_device: if output_device.is_empty() {
                    None
                } else {
                    Some(output_device.to_string())
                },
                input_device: if input_device.is_empty() {
                    None
                } else {
                    Some(input_device.to_string())
                },
            };

            let mut audio_io = state.audio_io.lock().map_err(|e| e.to_string())?;
            if audio_io.is_none() {
                *audio_io = Some(CpalAudioIO::new());
            }
            if let Some(ref mut io) = *audio_io {
                let cpu_load = Arc::clone(&state.cpu_load);
                io.start(eng_for_io, audio_config, param_rx, Some(cpu_load))
                    .map_err(|e| format!("Failed to start CPAL audio: {}", e))?;
            }
            tracing::info!(
                "Internal engine started with CPAL I/O ({} Hz, buffer {})",
                sample_rate,
                buffer_size
            );
        }
    }

    *state.active_mode.lock().map_err(|e| e.to_string())? = "internal".to_string();
    Ok(())
}

/// Stop the audio engine and all I/O backends.
#[tauri::command]
pub fn stop_engine(state: State<'_, AppState>) -> Result<(), String> {
    // ── Stop internal engine ──
    *state.param_tx.lock().map_err(|e| e.to_string())? = None;

    // Drop CPAL audio I/O first (this stops the stream and drops the callback)
    {
        let mut audio_io = state.audio_io.lock().map_err(|e| e.to_string())?;
        *audio_io = None;
    }

    // Drop JACK audio I/O first
    {
        let mut jack_io = state.jack_audio_io.lock().map_err(|e| e.to_string())?;
        *jack_io = None;
    }

    // Give the audio subsystem a moment to release the engine Arc
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Now safe to drop the engine
    let mut engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    *engine_guard = None;
    drop(engine_guard);

    // ── Stop Guitarix mode ──
    let mut client_guard = state.guitarix_client.lock().map_err(|e| e.to_string())?;
    if client_guard.is_some() {
        // Best-effort shutdown notification
        if let Some(ref mut client) = *client_guard {
            let _ = std::thread::spawn({
                let _client = client; // we can't easily async here, just drop
                || {}
            });
        }
        *client_guard = None;
    }
    drop(client_guard);

    let mut proc_guard = state.guitarix_process.lock().map_err(|e| e.to_string())?;
    if let Some(ref mut proc) = *proc_guard {
        let _ = proc.stop();
    }
    *proc_guard = None;

    *state.active_mode.lock().map_err(|e| e.to_string())? = "none".to_string();

    tracing::info!("Audio engine stopped");
    Ok(())
}

/// Get the current engine status.
#[tauri::command]
pub fn engine_status(state: State<'_, AppState>) -> EngineStatus {
    let mode = state
        .active_mode
        .lock()
        .map(|m| m.clone())
        .unwrap_or_else(|_| "unknown".to_string());

    let running = match mode.as_str() {
        "guitarix" => state.guitarix_client.lock().map(|c| c.is_some()).unwrap_or(false),
        "internal" => state
            .engine
            .lock()
            .map(|e| e.is_some())
            .unwrap_or(false),
        _ => false,
    };

    let (sr, bs, backend) = if let Ok(cfg) = state.config.try_lock() {
        (
            cfg.sample_rate as f64,
            cfg.buffer_size,
            format!("{:?}", cfg.audio_backend),
        )
    } else {
        (48000.0, 256, "Unknown".to_string())
    };

    EngineStatus {
        running,
        sample_rate: sr,
        buffer_size: bs,
        backend,
        mode,
    }
}

/// Get the DSP CPU load as a percentage (0.0 – 100.0+).
#[tauri::command]
pub fn get_cpu_load(state: State<'_, AppState>) -> f64 {
    let raw = state.cpu_load.load(std::sync::atomic::Ordering::Relaxed);
    raw as f64 / 1000.0
}

/// Set a named parameter on the active engine.
///
/// In **internal** mode this pushes to the lock-free SPSC parameter channel.
/// In **Guitarix** mode this sends the parameter via JSON-RPC.
#[tauri::command]
pub async fn set_parameter(
    state: State<'_, AppState>,
    id: String,
    value: f32,
) -> Result<(), String> {
    let mode = state
        .active_mode
        .lock()
        .map_err(|e| e.to_string())?
        .clone();

    match mode.as_str() {
        "guitarix" => {
            let client_opt = {
                let mut client_guard = state.guitarix_client.lock().map_err(|e| e.to_string())?;
                client_guard.take()
            };
            if let Some(mut client) = client_opt {
                client
                    .set_param(&id, value)
                    .await
                    .map_err(|e| format!("Guitarix RPC error: {}", e))?;
                *state.guitarix_client.lock().map_err(|e| e.to_string())? = Some(client);
            }
            Ok(())
        }
        _ => {
            // Internal mode: push to lock-free SPSC queue
            let tx_guard = state.param_tx.lock().map_err(|e| e.to_string())?;
            if let Some(ref tx) = *tx_guard {
                tx.send(id.clone(), value)
                    .map_err(|_| "Parameter queue full".to_string())?;
            }
            drop(tx_guard);

            // Opportunistically sync the engine's parameter HashMap
            if let Ok(engine_guard) = state.engine.try_lock() {
                if let Some(ref eng_arc) = *engine_guard {
                    if let Ok(mut eng) = eng_arc.try_lock() {
                        eng.set_parameter_value(&id, value);
                    }
                }
            }
            Ok(())
        }
    }
}

/// Get a parameter value from the active engine.
#[tauri::command]
pub async fn get_parameter(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<f32>, String> {
    let mode = state
        .active_mode
        .lock()
        .map_err(|e| e.to_string())?
        .clone();

    match mode.as_str() {
        "guitarix" => {
            let client_opt = {
                let mut client_guard = state.guitarix_client.lock().map_err(|e| e.to_string())?;
                client_guard.take()
            };
            if let Some(mut client) = client_opt {
                let val = client
                    .get_param(&id)
                    .await
                    .map_err(|e| format!("Guitarix RPC error: {}", e))?;
                *state.guitarix_client.lock().map_err(|e| e.to_string())? = Some(client);
                Ok(Some(val))
            } else {
                Err("Guitarix client not connected".to_string())
            }
        }
        _ => {
            let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
            if let Some(ref eng) = *engine_guard {
                let eng_inner = eng.lock().map_err(|e| e.to_string())?;
                Ok(eng_inner.get_parameter(&id))
            } else {
                Err("Engine not running".to_string())
            }
        }
    }
}

/// Get the current per-plugin RMS audio levels.
#[tauri::command]
pub fn get_audio_levels(state: State<'_, AppState>) -> Result<Vec<f32>, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let eng_inner = eng.lock().map_err(|e| e.to_string())?;
        Ok(eng_inner.audio_levels())
    } else {
        Err("Engine not running".to_string())
    }
}
