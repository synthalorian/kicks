use serde::{Deserialize, Serialize};

/// A mapping from a MIDI CC number to a parameter control.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiMapping {
    pub cc: u8,
    pub channel: u8,
    pub parameter_id: String,
    pub label: String,
    pub min: f32,
    pub max: f32,
}

/// The complete MIDI controller configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MidiConfig {
    /// Name of the active MIDI input device (None = no device selected).
    pub active_device: Option<String>,
    /// Global MIDI channel filter (0 = all channels).
    pub channel: u8,
    /// CC-to-parameter mappings.
    pub mappings: Vec<MidiMapping>,
    /// Whether MIDI learn mode is active.
    pub learn_mode: bool,
    /// The last CC number received (for learn mode UI).
    pub last_cc: Option<u8>,
}

impl Default for MidiConfig {
    fn default() -> Self {
        Self {
            active_device: None,
            channel: 1,
            mappings: Vec::new(),
            learn_mode: false,
            last_cc: None,
        }
    }
}
