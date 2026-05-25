use serde::{Deserialize, Serialize};

/// The complete signal chain: an ordered list of processing slots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalChain {
    pub slots: Vec<ChainSlot>,
}

/// A single slot in the signal chain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChainSlot {
    pub id: String,
    pub plugin_type: PluginType,
    pub enabled: bool,
    pub wet_dry: f32,
    pub parameters: std::collections::HashMap<String, f32>,
}

/// Types of plugins available in the signal chain.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginType {
    Input,
    Boost,
    Amp,
    /// Bass-specific amp with shifted EQ frequencies (100Hz/500Hz/4kHz).
    BassAmp,
    Cab,
    Delay,
    Reverb,
    Output,
    Custom(String),
}

impl Default for SignalChain {
    fn default() -> Self {
        Self {
            slots: vec![
                ChainSlot {
                    id: "input".to_string(),
                    plugin_type: PluginType::Input,
                    enabled: true,
                    wet_dry: 1.0,
                    parameters: std::collections::HashMap::new(),
                },
                ChainSlot {
                    id: "amp".to_string(),
                    plugin_type: PluginType::Amp,
                    enabled: true,
                    wet_dry: 1.0,
                    parameters: [("gain".into(), 0.5), ("master".into(), 0.7), ("bass".into(), 0.5), ("mid".into(), 0.5), ("treble".into(), 0.5), ("bass_mode".into(), 0.0)]
                        .into_iter().collect(),
                },
                ChainSlot {
                    id: "cab".to_string(),
                    plugin_type: PluginType::Cab,
                    enabled: true,
                    wet_dry: 1.0,
                    parameters: std::collections::HashMap::new(),
                },
                ChainSlot {
                    id: "output".to_string(),
                    plugin_type: PluginType::Output,
                    enabled: true,
                    wet_dry: 1.0,
                    parameters: [("volume".into(), 0.8)].into_iter().collect(),
                },
            ],
        }
    }
}
