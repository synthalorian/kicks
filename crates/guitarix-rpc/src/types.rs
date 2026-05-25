use serde::{Deserialize, Serialize};

/// A parameter as returned by `parameterlist` or `desc`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub id: String,
    pub name: String,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub group: String,
}

/// A bank of presets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub index: u32,
    pub name: String,
    pub presets: Vec<Preset>,
}

/// A single preset within a bank.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub index: u32,
    pub name: String,
}

/// A plugin unit loaded in the rack.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RackUnit {
    pub name: String,
    pub uri: String,
    pub enabled: bool,
}

/// MIDI controller mapping.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiControllerMap {
    pub channel: u8,
    pub mappings: Vec<MidiMapping>,
}

/// A single MIDI CC mapping to a parameter.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiMapping {
    pub cc: u8,
    pub parameter_id: String,
    pub min: f32,
    pub max: f32,
}
