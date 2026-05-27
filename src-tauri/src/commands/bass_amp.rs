use tauri::State;

use crate::AppState;

/// Switch the signal chain to bass mode (BassAmp instead of regular Amp).
#[tauri::command]
pub fn switch_to_bass_chain(state: State<'_, AppState>) -> Result<(), String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        eng_inner.build_bass_chain();
        tracing::info!("Switched to bass signal chain");
        Ok(())
    } else {
        Err("Engine not running".to_string())
    }
}

/// Switch the signal chain to practice mode (Tuner + Metronome + Amp + Cab).
#[tauri::command]
pub fn switch_to_practice_chain(state: State<'_, AppState>) -> Result<(), String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        eng_inner.build_practice_chain();
        tracing::info!("Switched to practice signal chain");
        Ok(())
    } else {
        Err("Engine not running".to_string())
    }
}

/// Switch the signal chain to looper mode.
#[tauri::command]
pub fn switch_to_looper_chain(state: State<'_, AppState>) -> Result<(), String> {
    let engine_guard = state.engine.lock().map_err(|e| e.to_string())?;
    if let Some(ref eng) = *engine_guard {
        let mut eng_inner = eng.lock().map_err(|e| e.to_string())?;
        eng_inner.build_looper_chain();
        tracing::info!("Switched to looper signal chain");
        Ok(())
    } else {
        Err("Engine not running".to_string())
    }
}
