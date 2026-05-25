use crate::client::GuitarixClient;
use crate::error::Result;
use serde_json::Value;

/// A parameter entry with ID, current value, and description.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParamInfo {
    pub id: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub label: String,
    pub group: String,
}

/// A preset entry with bank index, name, and position.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PresetEntry {
    pub bank_name: String,
    pub bank_index: u32,
    pub name: String,
    pub index: u32,
}

/// Enumerate all parameters with their current values and ranges.
///
/// Calls `parameterlist()` to get all parameter IDs, then fetches
/// the description (`desc()`) and value (`get()`) for each one.
pub async fn list_params(client: &mut GuitarixClient) -> Result<Vec<ParamInfo>> {
    let ids = client.parameter_list().await?;
    let mut params = Vec::with_capacity(ids.len());

    for id in ids {
        let value = client.get_param(&id).await.unwrap_or(0.0);
        let desc = client.desc(&id).await.unwrap_or(Value::Null);

        let (min, max, label, group) = parse_desc(&desc);

        params.push(ParamInfo {
            id,
            value,
            min,
            max,
            label,
            group,
        });
    }

    Ok(params)
}

/// Enumerate all presets across all banks.
///
/// Walks `banks()` → for each bank calls `presets(bank_name)`.
pub async fn list_all_presets(client: &mut GuitarixClient) -> Result<Vec<PresetEntry>> {
    let bank_names = client.banks().await?;
    let mut all = Vec::new();

    for (bank_idx, bank_name) in bank_names.iter().enumerate() {
        let preset_names = client.presets(bank_name).await?;
        for (preset_idx, preset_name) in preset_names.iter().enumerate() {
            all.push(PresetEntry {
                bank_name: bank_name.clone(),
                bank_index: bank_idx as u32,
                name: preset_name.clone(),
                index: preset_idx as u32,
            });
        }
    }

    Ok(all)
}

/// Load a specific preset from a bank.
pub async fn load_preset(client: &mut GuitarixClient, bank: &str, index: u32) -> Result<()> {
    client.set_preset(bank, index).await
}

/// Save current engine state as a new preset name.
pub async fn save_preset(client: &mut GuitarixClient, name: &str) -> Result<()> {
    client.save_preset(name).await
}

/// Create a new preset with the given name, appended after the last entry in the bank.
pub async fn create_preset(client: &mut GuitarixClient, name: &str) -> Result<()> {
    client.pf_append(name).await
}

/// Rename an existing preset.
pub async fn rename_preset(
    client: &mut GuitarixClient,
    bank: &str,
    index: u32,
    new_name: &str,
) -> Result<String> {
    client.rename_preset(bank, index, new_name).await
}

/// Delete a preset by bank and index.
pub async fn delete_preset(client: &mut GuitarixClient, bank: &str, index: u32) -> Result<()> {
    client.erase_preset(bank, index).await
}

/// Reorder a preset within a bank.
pub async fn reorder_preset(
    client: &mut GuitarixClient,
    bank: &str,
    from: u32,
    to: u32,
) -> Result<()> {
    client.reorder_preset(bank, from, to).await
}

/// Save the current engine state to the active bank file.
pub async fn save_bank(client: &mut GuitarixClient) -> Result<()> {
    client.bank_save().await
}

// ── Helpers ──

fn parse_desc(desc: &Value) -> (f32, f32, String, String) {
    let min = desc.get("min").and_then(|v| v.as_f64()).unwrap_or(0.0) as f32;
    let max = desc.get("max").and_then(|v| v.as_f64()).unwrap_or(1.0) as f32;
    let label = desc
        .get("label")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let group = desc
        .get("group")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    (min, max, label, group)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_desc_full() {
        let desc = serde_json::json!({
            "min": 0.0,
            "max": 10.0,
            "label": "Gain",
            "group": "Preamp",
        });
        let (min, max, label, group) = parse_desc(&desc);
        assert!((min - 0.0).abs() < f32::EPSILON);
        assert!((max - 10.0).abs() < f32::EPSILON);
        assert_eq!(label, "Gain");
        assert_eq!(group, "Preamp");
    }

    #[test]
    fn test_parse_desc_empty() {
        let desc = serde_json::json!({});
        let (min, max, label, group) = parse_desc(&desc);
        assert!((min - 0.0).abs() < f32::EPSILON);
        assert!((max - 1.0).abs() < f32::EPSILON);
        assert_eq!(label, "");
        assert_eq!(group, "");
    }
}
