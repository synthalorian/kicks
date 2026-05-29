use kicks_core::config::AiProvider;
use kicks_core::signal_chain::{ChainSlot, PluginType, SignalChain};
use serde::Deserialize;

/// The structured response we expect from the AI model.
#[derive(Debug, Deserialize)]
pub struct AiResponse {
    pub name: String,
    pub description: String,
    pub signal_chain: AiSignalChain,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AiSignalChain {
    pub(crate) slots: Vec<AiChainSlot>,
}

#[derive(Debug, Deserialize)]
pub(crate) struct AiChainSlot {
    plugin_type: String,
    enabled: bool,
    #[serde(default = "default_wet_dry")]
    wet_dry: f32,
    parameters: std::collections::HashMap<String, f32>,
}

fn default_wet_dry() -> f32 {
    1.0
}

pub fn parse_plugin_type(s: &str) -> PluginType {
    match s.to_lowercase().trim() {
        "input" => PluginType::Input,
        "boost" => PluginType::Boost,
        "amp" | "guitar" => PluginType::Amp,
        "bass" | "bassamp" | "bass_amp" => PluginType::BassAmp,
        "cab" => PluginType::Cab,
        "delay" => PluginType::Delay,
        "reverb" => PluginType::Reverb,
        "output" => PluginType::Output,
        other => PluginType::Custom(other.to_string()),
    }
}

fn build_system_prompt() -> String {
    r#"You are a guitar and bass tone expert. Given a natural-language description of a desired guitar or bass tone, generate a signal chain configuration for the Kicks guitar workstation.

Available plugin types and their valid parameters (all values 0.0 to 1.0 unless noted):

1. "Boost" — Clean boost / overdrive
   - gain (0..1 → 0..2x volume)

2. "Amp" — Guitar amplifier with 3-band EQ
   - gain (0..1 → input gain 0..10)
   - master (0..1 → output volume)
   - bass (0..1 → -12..+12 dB low shelf at 250Hz)
   - mid (0..1 → -12..+12 dB peaking at 800Hz)
   - treble (0..1 → -12..+12 dB high shelf at 3kHz)
   - drive (0..1 → 1..10x waveshaper drive)

3. "BassAmp" — Bass amplifier with 3-band EQ (shifted frequencies for bass)
   - gain (0..1 → input gain 0..10)
   - master (0..1 → output volume)
   - bass (0..1 → -12..+12 dB low shelf at 100Hz)
   - mid (0..1 → -12..+12 dB peaking at 500Hz)
   - treble (0..1 → -12..+12 dB high shelf at 4kHz)
   - drive (0..1 → 1..10x waveshaper drive)
   Use "BassAmp" plugin type when the description mentions bass guitar, low-end, sub, or deep tones.

4. "Cab" — Speaker cabinet simulation
   - level (0..1 → output level)
   - low_cut (0..1 → 20..250Hz highpass)
   - high_cut (0..1 → 8kHz..2kHz lowpass)

5. "Delay" — Digital delay with feedback
   - time (0..1 → 20ms..2000ms)
   - feedback (0..1 → 0..99%)
   - mix (0..1 → wet/dry)

6. "Reverb" — Schroeder reverberator
   - size (0..1 → room size)
   - damping (0..1 → high-frequency absorption)
   - mix (0..1 → wet/dry)

Output ONLY valid JSON with no markdown, no code fences, no explanation. Use this exact structure:
{
  "name": "Descriptive preset name",
  "description": "Short description of the tone",
  "signal_chain": {
    "slots": [
      { "plugin_type": "Amp", "enabled": true, "wet_dry": 1.0, "parameters": { "gain": 0.5, "master": 0.7, "bass": 0.5, "mid": 0.5, "treble": 0.5, "drive": 0.5 } },
      { "plugin_type": "Cab", "enabled": true, "wet_dry": 1.0, "parameters": { "level": 1.0, "low_cut": 0.0, "high_cut": 0.6 } }
    ]
  }
}

Only include plugins that are relevant to the described tone. Omit slots that don't contribute (e.g., skip Delay for a clean jazz tone with no delay). The frontend will add Input and Output slots automatically."#.to_string()
}

/// Parse the raw text response (strip fences) into an AiResponse.
fn parse_ai_text_response(text: &str) -> Result<AiResponse, String> {
    let cleaned = text
        .trim()
        .strip_prefix("```json")
        .or_else(|| text.trim().strip_prefix("```"))
        .map(|s| s.trim_end_matches("```").trim())
        .unwrap_or(text.trim());

    serde_json::from_str(cleaned)
        .map_err(|e| format!("Failed to parse AI output: {}.\nRaw: {}", e, cleaned))
}

/// Call the Anthropic Messages API.
async fn call_anthropic(
    description: &str, api_key: &str, model: &str, endpoint_url: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "system": build_system_prompt(),
        "messages": [
            {
                "role": "user",
                "content": format!("Generate a signal chain for this tone: {}", description)
            }
        ]
    });

    let resp = client
        .post(endpoint_url)
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .header("content-type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API returned {}: {}", status, text));
    }

    #[derive(Deserialize)]
    struct AnthropicMessage {
        content: Vec<AnthropicContent>,
    }

    #[derive(Deserialize)]
    struct AnthropicContent {
        text: Option<String>,
    }

    let msg: AnthropicMessage = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    msg.content
        .into_iter()
        .find_map(|c| c.text)
        .ok_or_else(|| "API returned empty response".to_string())
}

/// Call an OpenAI-compatible chat completions endpoint.
/// Used by: OpenAI, OpenRouter, Ollama, llama-server, llama-swap, GLM, Kimi, Together, Groq, etc.
async fn call_openai(
    description: &str, api_key: &str, model: &str, endpoint_url: &str,
) -> Result<String, String> {
    let client = reqwest::Client::new();

    let body = serde_json::json!({
        "model": model,
        "max_tokens": 1024,
        "temperature": 0.7,
        "messages": [
            {
                "role": "system",
                "content": build_system_prompt()
            },
            {
                "role": "user",
                "content": format!("Generate a signal chain for this tone: {}", description)
            }
        ]
    });

    let mut req = client
        .post(endpoint_url)
        .header("content-type", "application/json");

    // Only add auth header if an API key is provided (for local models like llama-server that don't need one)
    if !api_key.is_empty() {
        req = req.header("Authorization", format!("Bearer {}", api_key));
    }

    let resp = req
        .json(&body)
        .send()
        .await
        .map_err(|e| format!("API request failed: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let text = resp.text().await.unwrap_or_default();
        return Err(format!("API returned {}: {}", status, text));
    }

    #[derive(Deserialize)]
    struct OpenAiResponse {
        choices: Vec<OpenAiChoice>,
    }

    #[derive(Deserialize)]
    struct OpenAiChoice {
        message: OpenAiMessage,
    }

    #[derive(Deserialize)]
    struct OpenAiMessage {
        content: Option<String>,
    }

    let msg: OpenAiResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse API response: {}", e))?;

    msg.choices
        .into_iter()
        .next()
        .and_then(|c| c.message.content)
        .ok_or_else(|| "API returned empty response".to_string())
}

/// Generate an AI tone preset by routing to the configured provider.
/// Supports Anthropic and OpenAI-compatible API formats.
pub async fn generate_preset(
    description: &str, api_key: &str, model: &str, provider: &AiProvider, endpoint_url: &str,
) -> Result<AiResponse, String> {
    let text = match provider {
        AiProvider::Anthropic => {
            if api_key.is_empty() {
                return Err("No API key configured. Add your API key in Settings.".to_string());
            }
            call_anthropic(description, api_key, model, endpoint_url).await?
        }
        AiProvider::OpenAI => call_openai(description, api_key, model, endpoint_url).await?,
    };

    parse_ai_text_response(&text)
}

/// Convert an AiResponse into a full SignalChain (with Input/Output slots).
#[allow(dead_code)]
pub fn ai_response_to_signal_chain(resp: AiResponse) -> SignalChain {
    let mut slots = Vec::new();

    // Input slot
    slots.push(ChainSlot {
        id: "input".to_string(),
        plugin_type: PluginType::Input,
        enabled: true,
        wet_dry: 1.0,
        parameters: std::collections::HashMap::new(),
    });

    // AI-generated slots
    for (i, s) in resp.signal_chain.slots.into_iter().enumerate() {
        let plugin_type = parse_plugin_type(&s.plugin_type);
        let type_name = format!("{:?}", plugin_type).to_lowercase();
        slots.push(ChainSlot {
            id: format!("{}-{}", type_name, i),
            plugin_type,
            enabled: s.enabled,
            wet_dry: s.wet_dry.clamp(0.0, 1.0),
            parameters: s
                .parameters
                .into_iter()
                .map(|(k, v)| (k, v.clamp(0.0, 1.0)))
                .collect(),
        });
    }

    // Output slot
    slots.push(ChainSlot {
        id: "output".to_string(),
        plugin_type: PluginType::Output,
        enabled: true,
        wet_dry: 1.0,
        parameters: [("volume".to_string(), 0.8)].into_iter().collect(),
    });

    SignalChain { slots }
}

/// The result sent to the frontend.
#[derive(Debug, serde::Serialize)]
pub struct AiPresetResult {
    pub name: String,
    pub description: String,
    pub signal_chain: AiSignalChainInfo,
}

#[derive(Debug, serde::Serialize)]
pub struct AiSlotInfo {
    pub plugin_type: String,
    pub enabled: bool,
    pub wet_dry: f32,
    pub parameters: std::collections::HashMap<String, f32>,
}

#[derive(Debug, serde::Serialize)]
pub struct AiSignalChainInfo {
    pub slots: Vec<AiSlotInfo>,
}

impl From<AiResponse> for AiPresetResult {
    fn from(r: AiResponse) -> Self {
        Self {
            name: r.name,
            description: r.description,
            signal_chain: AiSignalChainInfo {
                slots: r
                    .signal_chain
                    .slots
                    .into_iter()
                    .map(|s| AiSlotInfo {
                        plugin_type: s.plugin_type,
                        enabled: s.enabled,
                        wet_dry: s.wet_dry,
                        parameters: s.parameters,
                    })
                    .collect(),
            },
        }
    }
}
