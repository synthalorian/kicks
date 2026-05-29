use kicks_core::persistence;
use tauri::State;

use crate::AppState;

/// Generate an AI tone preset from a natural language description.
#[tauri::command]
pub async fn generate_ai_preset(
    state: State<'_, AppState>, description: String,
) -> Result<crate::ai::AiPresetResult, String> {
    let (api_key, model, provider, endpoint_url) = {
        let config = state.config.lock().map_err(|e| e.to_string())?;
        (
            config.ai_api_key.clone(),
            config.ai_model.clone(),
            config.ai_provider.clone(),
            config.ai_endpoint_url.clone(),
        )
    };

    let ai_resp =
        crate::ai::generate_preset(&description, &api_key, &model, &provider, &endpoint_url)
            .await?;

    tracing::info!(
        "AI generated preset: '{}' (provider: {:?})",
        ai_resp.name,
        provider
    );
    Ok(crate::ai::AiPresetResult::from(ai_resp))
}

/// Apply an AI-generated preset to the current signal chain.
#[tauri::command]
pub fn apply_ai_preset(
    state: State<'_, AppState>, signal_chain: AiPresetPayload,
) -> Result<(), String> {
    let mut chain = state.signal_chain.lock().map_err(|e| e.to_string())?;

    let mut slots = Vec::new();

    // Input
    slots.push(kicks_core::signal_chain::ChainSlot {
        id: "input".to_string(),
        plugin_type: kicks_core::signal_chain::PluginType::Input,
        enabled: true,
        wet_dry: 1.0,
        parameters: std::collections::HashMap::new(),
    });

    for (i, slot) in signal_chain.slots.into_iter().enumerate() {
        let plugin_type = crate::ai::parse_plugin_type(&slot.plugin_type);
        let type_name = format!("{:?}", plugin_type).to_lowercase();
        slots.push(kicks_core::signal_chain::ChainSlot {
            id: format!("{}-{}", type_name, i),
            plugin_type,
            enabled: slot.enabled,
            wet_dry: slot.wet_dry.clamp(0.0, 1.0),
            parameters: slot
                .parameters
                .into_iter()
                .map(|(k, v)| (k, v.clamp(0.0, 1.0)))
                .collect(),
        });
    }

    // Output
    slots.push(kicks_core::signal_chain::ChainSlot {
        id: "output".to_string(),
        plugin_type: kicks_core::signal_chain::PluginType::Output,
        enabled: true,
        wet_dry: 1.0,
        parameters: [("volume".to_string(), 0.8)].into_iter().collect(),
    });

    chain.slots = slots;

    // Persist
    if let Err(e) = persistence::save_signal_chain(&chain) {
        tracing::warn!("Failed to persist signal chain after AI apply: {}", e);
    }

    tracing::info!("AI preset applied to signal chain");
    Ok(())
}

/// Payload for applying an AI preset to the signal chain.
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AiPresetPayload {
    pub slots: Vec<AiSlotPayload>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AiSlotPayload {
    pub plugin_type: String,
    pub enabled: bool,
    pub wet_dry: f32,
    pub parameters: std::collections::HashMap<String, f32>,
}
