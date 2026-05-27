use serde::{Deserialize, Serialize};

use crate::signal_chain::SignalChain;

/// A named preset containing a complete signal chain configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Preset {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub signal_chain: SignalChain,
    pub created: chrono::DateTime<chrono::Utc>,
    pub modified: chrono::DateTime<chrono::Utc>,
}

/// A collection of presets, organized into banks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PresetCollection {
    pub banks: Vec<Bank>,
}

/// A named bank containing presets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bank {
    pub name: String,
    pub presets: Vec<Preset>,
}
