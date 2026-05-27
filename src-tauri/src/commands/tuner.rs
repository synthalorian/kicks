use serde::Serialize;
use tauri::State;

use crate::AppState;

/// Current tuner detection result.
#[derive(Debug, Serialize)]
pub struct TunerResult {
    pub frequency: f32,
    pub note: String,
    pub cents: f32,
    pub confidence: f32,
    pub active: bool,
}

/// Get the current tuner detection info.
#[tauri::command]
pub fn get_tuner_info(state: State<'_, AppState>) -> Result<TunerResult, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let eng_inner = eng.lock().map_err(|e| e.to_string())?;
        if let Some((freq, note, cents, conf)) = eng_inner.tuner_info() {
            Ok(TunerResult {
                frequency: freq,
                note,
                cents,
                confidence: conf,
                active: conf > 0.3,
            })
        } else {
            Ok(TunerResult {
                frequency: 0.0,
                note: "--".to_string(),
                cents: 0.0,
                confidence: 0.0,
                active: false,
            })
        }
    } else {
        Err("Engine not running".to_string())
    }
}
