use serde::{Deserialize, Serialize};

/// AI provider selection.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AiProvider {
    #[default]
    Anthropic,
    OpenAI,
}

impl AiProvider {
    pub fn default_endpoint(&self) -> String {
        match self {
            Self::Anthropic => "https://api.anthropic.com/v1/messages".to_string(),
            Self::OpenAI => "https://api.openai.com/v1/chat/completions".to_string(),
        }
    }
}

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KicksConfig {
    /// Guitarix RPC connection settings.
    pub guitarix_host: String,
    pub guitarix_port: u16,

    /// Audio engine selection.
    pub active_engine: EngineMode,

    /// JACK client name.
    pub jack_client_name: String,

    /// CPAL audio device settings.
    pub sample_rate: u32,
    pub buffer_size: u32,
    /// Empty string = use system default device.
    pub audio_device: String,
    /// Separate input/output device selection (replaces single audio_device).
    pub input_device: String,
    pub output_device: String,

    /// Audio backend selection.
    pub audio_backend: AudioBackend,
    pub ir_directories: Vec<String>,
    pub nam_directories: Vec<String>,
    pub preset_directories: Vec<String>,

    /// AI tone assistant settings.
    pub ai_provider: AiProvider,
    pub ai_endpoint_url: String,
    pub ai_api_key: String,
    pub ai_model: String,

    /// Path to the last loaded IR file (restored on startup).
    pub active_ir_path: String,
}

#[allow(clippy::derivable_impls)]
impl Default for KicksConfig {
    fn default() -> Self {
        Self {
            guitarix_host: "127.0.0.1".to_string(),
            guitarix_port: 4040,
            active_engine: EngineMode::Guitarix,
            jack_client_name: "kicks".to_string(),
            sample_rate: 48000,
            buffer_size: 256,
            audio_device: String::new(),
            input_device: String::new(),
            output_device: String::new(),
            audio_backend: AudioBackend::default(),
            ir_directories: vec![
                std::env::var("HOME").unwrap_or_default() + "/.config/guitarix/impulses",
                std::env::var("HOME").unwrap_or_default() + "/IR",
            ],
            nam_directories: vec![std::env::var("HOME").unwrap_or_default() + "/.nam"],
            preset_directories: vec![
                std::env::var("HOME").unwrap_or_default() + "/.config/kicks/presets",
            ],
            ai_provider: AiProvider::default(),
            ai_endpoint_url: AiProvider::default().default_endpoint(),
            ai_api_key: String::new(),
            ai_model: "claude-sonnet-4-20250514".to_string(),
            active_ir_path: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EngineMode {
    Guitarix,
    Internal,
    Auto,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub enum AudioBackend {
    #[default]
    Cpal,
    Jack,
}
