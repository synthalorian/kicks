use serde::Serialize;
use tauri::State;

use crate::AppState;

/// Current looper state.
#[derive(Debug, Serialize)]
pub struct LooperState {
    pub mode: String,
    pub loop_time_seconds: f32,
    pub has_loop: bool,
}

/// Get the current looper state.
#[tauri::command]
pub fn get_looper_state(state: State<'_, AppState>) -> Result<LooperState, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let eng_inner = eng.lock().map_err(|e| e.to_string())?;
        if let Some((mode, time, has_loop)) = eng_inner.looper_state() {
            Ok(LooperState {
                mode,
                loop_time_seconds: time,
                has_loop,
            })
        } else {
            Ok(LooperState {
                mode: "idle".to_string(),
                loop_time_seconds: 0.0,
                has_loop: false,
            })
        }
    } else {
        Err("Engine not running".to_string())
    }
}

/// Trigger a looper mode change.
/// Mode values: 0=idle, 1=record, 2=overdub, 3=play, 4=stop
#[tauri::command]
pub fn trigger_looper_mode(state: State<'_, AppState>, mode: u8) -> Result<bool, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        Ok(eng_inner.trigger_looper_mode(mode as f32))
    } else {
        Err("Engine not running".to_string())
    }
}

/// Undo the last looper overdub.
#[tauri::command]
pub fn looper_undo(state: State<'_, AppState>) -> Result<bool, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        Ok(eng_inner.looper_undo())
    } else {
        Err("Engine not running".to_string())
    }
}

/// Clear the looper buffer.
#[tauri::command]
pub fn looper_clear(state: State<'_, AppState>) -> Result<bool, String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        Ok(eng_inner.looper_clear())
    } else {
        Err("Engine not running".to_string())
    }
}
