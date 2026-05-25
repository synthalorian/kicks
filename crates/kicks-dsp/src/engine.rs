use crate::plugins::{IrInfo, PluginRegistry};

/// The trait each audio engine backend must implement.
pub trait AudioEngine: Send {
    /// Initialize the engine with the given sample rate and buffer size.
    fn init(&mut self, sample_rate: f64, buffer_size: u32) -> anyhow::Result<()>;

    /// Process one buffer of audio (interleaved stereo or mono).
    fn process(&mut self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()>;

    /// Shut down the engine and release resources.
    fn shutdown(&mut self) -> anyhow::Result<()>;

    /// Get the current value of a named parameter.
    fn get_parameter(&self, id: &str) -> Option<f32>;

    /// Set a named parameter, with smooth transition.
    fn set_parameter(&mut self, id: &str, value: f32);
}

/// The default internal processing engine using kicks-dsp plugins.
pub struct KicksEngine {
    plugin_registry: PluginRegistry,
    sample_rate: f64,
    buffer_size: u32,
}

impl KicksEngine {
    pub fn new() -> Self {
        let mut registry = PluginRegistry::new();
        registry.build_default_chain();
        Self {
            plugin_registry: registry,
            sample_rate: 48000.0,
            buffer_size: 256,
        }
    }

    /// Update only the parameter value cache (HashMap) without iterating plugins.
    /// Used by the main thread for immediate `get_parameter` consistency.
    /// The audio callback applies the full plugin update via the SPSC queue drain.
    pub fn set_parameter_value(&mut self, id: &str, value: f32) {
        self.plugin_registry.set_parameter_value(id, value);
    }

    /// Return the current per-plugin RMS audio levels (0..1 range).
    /// Updated every `process_all` cycle (~5 ms).
    pub fn audio_levels(&self) -> Vec<f32> {
        self.plugin_registry.audio_levels()
    }

    /// Load an impulse response into the Cab plugin.
    pub fn load_ir_to_cab(&mut self, path: String, ir_data: Vec<f32>, ir_sample_rate: f32) -> bool {
        self.plugin_registry.load_ir_to_cab(path, ir_data, ir_sample_rate)
    }

    /// Get info about the currently loaded IR in the Cab plugin.
    pub fn cab_ir_info(&self) -> Option<IrInfo> {
        self.plugin_registry.cab_ir_info()
    }

    /// Clear the loaded IR from the Cab plugin.
    pub fn clear_cab_ir(&mut self) -> bool {
        self.plugin_registry.clear_cab_ir()
    }
}

impl AudioEngine for KicksEngine {
    fn init(&mut self, sample_rate: f64, buffer_size: u32) -> anyhow::Result<()> {
        self.sample_rate = sample_rate;
        self.buffer_size = buffer_size;
        self.plugin_registry.init_all(sample_rate)?;
        tracing::info!(
            "KicksEngine initialized at {} Hz, buffer size {}",
            sample_rate,
            buffer_size
        );
        Ok(())
    }

    fn process(&mut self, input: &[f32], output: &mut [f32]) -> anyhow::Result<()> {
        self.plugin_registry.process_all(input, output)
    }

    fn shutdown(&mut self) -> anyhow::Result<()> {
        tracing::info!("KicksEngine shutting down");
        Ok(())
    }

    fn get_parameter(&self, id: &str) -> Option<f32> {
        self.plugin_registry.get_parameter(id)
    }

    fn set_parameter(&mut self, id: &str, value: f32) {
        self.plugin_registry.set_parameter(id, value);
    }
}
