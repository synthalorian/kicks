use serde::Serialize;
use tauri::State;

use crate::AppState;

/// Current metronome state.
#[derive(Debug, Serialize)]
pub struct MetronomeState {
    pub bpm: f32,
    pub beats_per_bar: u8,
    pub running: bool,
}

/// Get the current metronome state.
#[tauri::command]
pub fn get_metronome_state(state: State<'_, AppState>) -> Result<MetronomeState, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let eng_inner = eng.lock().map_err(|e| e.to_string())?;
        if let Some((bpm, beats, running)) = eng_inner.metronome_state() {
            Ok(MetronomeState { bpm, beats_per_bar: beats, running })
        } else {
            Ok(MetronomeState {
                bpm: 120.0,
                beats_per_bar: 4,
                running: false,
            })
        }
    } else {
        Err("Engine not running".to_string())
    }
}
