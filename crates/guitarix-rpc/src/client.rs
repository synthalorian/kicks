use serde_json::{json, Value};
use std::sync::atomic::{AtomicU64, Ordering};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::error::Result;

/// A JSON-RPC 2.0 client for communicating with a running Guitarix engine.
pub struct GuitarixClient {
    stream: Mutex<BufReader<TcpStream>>,
    request_id: AtomicU64,
}

impl GuitarixClient {
    pub async fn connect(host: &str, port: u16) -> Result<Self> {
        let addr = format!("{}:{}", host, port);
        let stream = TcpStream::connect(&addr).await?;
        Ok(Self {
            stream: Mutex::new(BufReader::new(stream)),
            request_id: AtomicU64::new(1),
        })
    }

    pub async fn call(&mut self, method: &str, params: Value) -> Result<Value> {
        let id = self.request_id.fetch_add(1, Ordering::SeqCst);
        let request = serde_json::json!({
            "jsonrpc": "2.0", "method": method, "params": params, "id": id,
        });
        let mut raw = serde_json::to_string(&request)?;
        raw.push('\n');
        let mut stream = self.stream.lock().await;
        stream.write_all(raw.as_bytes()).await?;
        stream.flush().await?;
        let mut line = String::new();
        stream.read_line(&mut line).await?;
        if line.is_empty() {
            return Err(crate::error::Error::ConnectionClosed);
        }
        let response: Value = serde_json::from_str(&line)?;
        if let Some(error) = response.get("error") {
            return Err(crate::error::Error::RpcError(error.to_string()));
        }
        Ok(response["result"].clone())
    }

    pub async fn notify(&mut self, method: &str, params: Value) -> Result<()> {
        let request = serde_json::json!({
            "jsonrpc": "2.0", "method": method, "params": params,
        });
        let mut raw = serde_json::to_string(&request)?;
        raw.push('\n');
        let mut stream = self.stream.lock().await;
        stream.write_all(raw.as_bytes()).await?;
        stream.flush().await?;
        Ok(())
    }

    // ── Server ──

    pub async fn get_version(&mut self) -> Result<String> {
        let v = self.call("getversion", json!([])).await?;
        Ok(v.as_str().unwrap_or_default().to_string())
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.notify("shutdown", json!([])).await
    }

    pub async fn listen(&mut self) -> Result<()> {
        self.notify("listen", json!([])).await
    }

    pub async fn unlisten(&mut self) -> Result<()> {
        self.notify("unlisten", json!([])).await
    }

    // ── Engine ──

    pub async fn get_state(&mut self) -> Result<Value> {
        self.call("getstate", json!([])).await
    }

    pub async fn set_state(&mut self, state: Value) -> Result<()> {
        self.notify("setstate", state).await
    }

    pub async fn jack_cpu_load(&mut self) -> Result<f32> {
        let v = self.call("jack_cpu_load", json!([])).await?;
        Ok(v.as_f64().unwrap_or(0.0) as f32)
    }

    pub async fn set_jack_insert(&mut self, enable: bool) -> Result<()> {
        self.notify("set_jack_insert", json!([enable])).await
    }

    // ── Parameters ──

    pub async fn get_param(&mut self, id: &str) -> Result<f32> {
        let v = self.call("get", json!([id])).await?;
        Ok(v.as_f64().unwrap_or(0.0) as f32)
    }

    pub async fn set_param(&mut self, id: &str, value: f32) -> Result<()> {
        self.notify("set", json!([id, value])).await
    }

    pub async fn parameter_list(&mut self) -> Result<Vec<String>> {
        let v = self.call("parameterlist", json!([])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn get_parameter(&mut self, id: &str) -> Result<Value> {
        self.call("get_parameter", json!([id])).await
    }

    pub async fn get_parameter_value(&mut self, id: &str) -> Result<f32> {
        let v = self.call("get_parameter_value", json!([id])).await?;
        Ok(v.as_f64().unwrap_or(0.0) as f32)
    }

    pub async fn desc(&mut self, id: &str) -> Result<Value> {
        self.call("desc", json!([id])).await
    }

    pub async fn list(&mut self, group: &str) -> Result<Vec<String>> {
        let v = self.call("list", json!([group])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    // ── Banks ──

    pub async fn banks(&mut self) -> Result<Vec<String>> {
        let v = self.call("banks", json!([])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn get_bank(&mut self, index: u32) -> Result<Value> {
        self.call("get_bank", json!([index])).await
    }

    pub async fn bank_get_contents(&mut self, index: u32) -> Result<Vec<String>> {
        let v = self.call("bank_get_contents", json!([index])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn bank_insert_content(&mut self, index: u32, name: &str) -> Result<()> {
        self.notify("bank_insert_content", json!([index, name]))
            .await
    }

    pub async fn bank_insert_new(&mut self, index: u32, name: &str) -> Result<String> {
        let v = self.call("bank_insert_new", json!([index, name])).await?;
        Ok(v.as_str().unwrap_or_default().to_string())
    }

    pub async fn rename_bank(&mut self, old: &str, new: &str) -> Result<String> {
        self.call("rename_bank", json!([old, new]))
            .await
            .map(|v| v.as_str().unwrap_or_default().to_string())
    }

    pub async fn bank_remove(&mut self, index: u32) -> Result<bool> {
        let v = self.call("bank_remove", json!([index])).await?;
        Ok(v.as_bool().unwrap_or(false))
    }

    pub async fn bank_reorder(&mut self, from: u32, to: u32) -> Result<()> {
        self.notify("bank_reorder", json!([from, to])).await
    }

    pub async fn bank_check_reparse(&mut self) -> Result<Value> {
        self.call("bank_check_reparse", json!([])).await
    }

    pub async fn bank_get_filename(&mut self, index: u32) -> Result<String> {
        let v = self.call("bank_get_filename", json!([index])).await?;
        Ok(v.as_str().unwrap_or_default().to_string())
    }

    pub async fn bank_set_flag(&mut self, index: u32, flag: bool) -> Result<()> {
        self.notify("bank_set_flag", json!([index, flag])).await
    }

    pub async fn convert_preset(&mut self, bank: &str, index: u32) -> Result<String> {
        self.call("convert_preset", json!([bank, index]))
            .await
            .map(|v| v.as_str().unwrap_or_default().to_string())
    }

    pub async fn bank_save(&mut self) -> Result<()> {
        self.notify("bank_save", json!([])).await
    }

    pub async fn save_current(&mut self) -> Result<()> {
        self.notify("save_current", json!([])).await
    }

    pub async fn save_preset(&mut self, name: &str) -> Result<()> {
        self.notify("save_preset", json!([name])).await
    }

    pub async fn pf_save(&mut self) -> Result<()> {
        self.notify("pf_save", json!([])).await
    }

    // ── Presets ──

    pub async fn presets(&mut self, bank: &str) -> Result<Vec<String>> {
        let v = self.call("presets", json!([bank])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn set_preset(&mut self, bank: &str, index: u32) -> Result<()> {
        self.notify("setpreset", json!([bank, index])).await
    }

    pub async fn create_default_scratch_preset(&mut self) -> Result<()> {
        self.notify("create_default_scratch_preset", json!([]))
            .await
    }

    pub async fn rename_preset(&mut self, bank: &str, index: u32, name: &str) -> Result<String> {
        self.call("rename_preset", json!([bank, index, name]))
            .await
            .map(|v| v.as_str().unwrap_or_default().to_string())
    }

    pub async fn reorder_preset(&mut self, bank: &str, from: u32, to: u32) -> Result<()> {
        self.notify("reorder_preset", json!([bank, from, to])).await
    }

    pub async fn erase_preset(&mut self, bank: &str, index: u32) -> Result<()> {
        self.notify("erase_preset", json!([bank, index])).await
    }

    pub async fn pf_append(&mut self, name: &str) -> Result<()> {
        self.notify("pf_append", json!([name])).await
    }

    pub async fn pf_insert_before(&mut self, name: &str) -> Result<()> {
        self.notify("pf_insert_before", json!([name])).await
    }

    pub async fn pf_insert_after(&mut self, name: &str) -> Result<()> {
        self.notify("pf_insert_after", json!([name])).await
    }

    // ── Unit Presets ──

    pub async fn plugin_preset_list_load(&mut self) -> Result<Value> {
        self.call("plugin_preset_list_load", json!([])).await
    }

    pub async fn plugin_preset_list_sync_set(&mut self, data: Value) -> Result<()> {
        self.notify("plugin_preset_list_sync_set", data).await
    }

    pub async fn plugin_preset_list_set(&mut self, data: Value) -> Result<()> {
        self.notify("plugin_preset_list_set", data).await
    }

    pub async fn plugin_preset_list_save(&mut self) -> Result<()> {
        self.notify("plugin_preset_list_save", json!([])).await
    }

    pub async fn plugin_preset_list_remove(&mut self, name: &str) -> Result<()> {
        self.notify("plugin_preset_list_remove", json!([name]))
            .await
    }

    // ── Plugins / Rack ──

    pub async fn plugin_list(&mut self) -> Result<Vec<String>> {
        let v = self.call("pluginlist", json!([])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn plugin_load_ui(&mut self, name: &str) -> Result<Value> {
        self.call("plugin_load_ui", json!([name])).await
    }

    pub async fn get_rack_unit_order(&mut self) -> Result<Vec<String>> {
        let v = self.call("get_rack_unit_order", json!([])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn get_file_list(&mut self, path: &str) -> Result<Vec<String>> {
        let v = self.call("get_file_list", json!([path])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn insert_rack_unit(&mut self, name: &str) -> Result<()> {
        self.notify("insert_rack_unit", json!([name])).await
    }

    pub async fn remove_rack_unit(&mut self, name: &str) -> Result<()> {
        self.notify("remove_rack_unit", json!([name])).await
    }

    pub async fn query_unit(&mut self, name: &str) -> Result<Value> {
        self.call("queryunit", json!([name])).await
    }

    // ── MIDI ──

    pub async fn get_midi_controller_map(&mut self) -> Result<Value> {
        self.call("get_midi_controller_map", json!([])).await
    }

    pub async fn midi_size(&mut self) -> Result<u32> {
        let v = self.call("midi_size", json!([])).await?;
        Ok(v.as_u64().unwrap_or(0) as u32)
    }

    pub async fn midi_delete_parameter(&mut self, idx: u32) -> Result<()> {
        self.notify("midi_deleteParameter", json!([idx])).await
    }

    pub async fn midi_modify_current(&mut self, data: Value) -> Result<()> {
        self.notify("midi_modifyCurrent", data).await
    }

    pub async fn midi_get_config_mode(&mut self) -> Result<bool> {
        let v = self.call("midi_get_config_mode", json!([])).await?;
        Ok(v.as_bool().unwrap_or(false))
    }

    pub async fn midi_set_config_mode(&mut self, on: bool) -> Result<()> {
        self.notify("midi_set_config_mode", json!([on])).await
    }

    pub async fn midi_set_current_control(&mut self, cc: u8) -> Result<()> {
        self.notify("midi_set_current_control", json!([cc])).await
    }

    pub async fn set_midi_channel(&mut self, ch: u8) -> Result<()> {
        self.notify("set_midi_channel", json!([ch])).await
    }

    pub async fn request_midi_value_update(&mut self) -> Result<()> {
        self.notify("request_midi_value_update", json!([])).await
    }

    pub async fn get_last_midi_control_value(&mut self) -> Result<f32> {
        let v = self.call("get_last_midi_control_value", json!([])).await?;
        Ok(v.as_f64().unwrap_or(0.0) as f32)
    }

    pub async fn set_last_midi_control_value(&mut self, value: f32) -> Result<()> {
        self.notify("set_last_midi_control_value", json!([value]))
            .await
    }

    pub async fn get_midi_feedback(&mut self) -> Result<Value> {
        self.call("get_midi_feedback", json!([])).await
    }

    pub async fn set_midi_feedback(&mut self, flag: bool) -> Result<()> {
        self.notify("set_midi_feedback", json!([flag])).await
    }

    // ── Tuner ──

    pub async fn get_tuning(&mut self) -> Result<f32> {
        let v = self.call("get_tuning", json!([])).await?;
        Ok(v.as_f64().unwrap_or(440.0) as f32)
    }

    pub async fn get_tuner_freq(&mut self) -> Result<f32> {
        let v = self.call("get_tuner_freq", json!([])).await?;
        Ok(v.as_f64().unwrap_or(0.0) as f32)
    }

    pub async fn get_tuner_note(&mut self) -> Result<String> {
        let v = self.call("get_tuner_note", json!([])).await?;
        Ok(v.as_str().unwrap_or_default().to_string())
    }

    pub async fn switch_tuner(&mut self, on: bool) -> Result<()> {
        self.notify("switch_tuner", json!([on])).await
    }

    pub async fn tuner_used_for_display(&mut self, val: bool) -> Result<()> {
        self.notify("tuner_used_for_display", json!([val])).await
    }

    pub async fn tuner_used_by_midi(&mut self, val: bool) -> Result<()> {
        self.notify("tuner_used_by_midi", json!([val])).await
    }

    // ── Oscilloscope ──

    pub async fn set_oscilloscope_mul_buffer(&mut self, val: u32) -> Result<()> {
        self.notify("set_oscilloscope_mul_buffer", json!([val]))
            .await
    }

    pub async fn get_oscilloscope_mul_buffer(&mut self) -> Result<Vec<f32>> {
        let v = self.call("get_oscilloscope_mul_buffer", json!([])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_f64().map(|f| f as f32))
                    .collect()
            })
            .unwrap_or_default())
    }

    // ── Convolver ──

    pub async fn reload_impresp_list(&mut self) -> Result<()> {
        self.notify("reload_impresp_list", json!([])).await
    }

    pub async fn load_impresp_dirs(&mut self) -> Result<Vec<String>> {
        let v = self.call("load_impresp_dirs", json!([])).await?;
        Ok(v.as_array()
            .map(|a| {
                a.iter()
                    .filter_map(|x| x.as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default())
    }

    pub async fn read_audio(&mut self, path: &str) -> Result<Value> {
        self.call("read_audio", json!([path])).await
    }

    // ── LADSPA ──

    pub async fn load_ladspa_list(&mut self) -> Result<Value> {
        self.call("load_ladspalist", json!([])).await
    }

    pub async fn save_ladspa_list(&mut self, data: Value) -> Result<()> {
        self.notify("save_ladspalist", data).await
    }

    pub async fn ladspaloader_update_plugins(&mut self) -> Result<Value> {
        self.call("ladspaloader_update_plugins", json!([])).await
    }

    // ── Tuner Switcher ──

    pub async fn get_tuner_switcher_active(&mut self) -> Result<bool> {
        let v = self.call("get_tuner_switcher_active", json!([])).await?;
        Ok(v.as_bool().unwrap_or(false))
    }

    pub async fn tuner_switcher_activate(&mut self) -> Result<()> {
        self.notify("tuner_switcher_activate", json!([])).await
    }

    pub async fn tuner_switcher_deactivate(&mut self) -> Result<()> {
        self.notify("tuner_switcher_deactivate", json!([])).await
    }

    pub async fn tuner_switcher_toggle(&mut self) -> Result<()> {
        self.notify("tuner_switcher_toggle", json!([])).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    /// Helper: spawns a tiny JSON-RPC echo server on localhost.
    /// Returns the port it bound to.
    async fn spawn_echo_server() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 4096];
            let n = stream.read(&mut buf).await.unwrap();
            let request: Value = serde_json::from_slice(&buf[..n]).unwrap();

            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "result": "ok",
                "id": request.get("id").and_then(|v| v.as_u64()).unwrap_or(0),
            });
            let mut resp = serde_json::to_string(&response).unwrap();
            resp.push('\n');
            stream.write_all(resp.as_bytes()).await.unwrap();
        });
        port
    }

    #[tokio::test]
    async fn test_call() {
        let port = spawn_echo_server().await;
        let mut client = GuitarixClient::connect("127.0.0.1", port).await.unwrap();
        let result = client.call("test_method", json!(["arg1"])).await.unwrap();
        assert_eq!(result, json!("ok"));
    }

    #[tokio::test]
    async fn test_notify() {
        let port = spawn_echo_server().await;
        let mut client = GuitarixClient::connect("127.0.0.1", port).await.unwrap();
        client.notify("test_notify", json!(["arg1"])).await.unwrap();
    }

    #[tokio::test]
    async fn test_call_round_trip() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let mut buf = vec![0u8; 4096];
            let n = stream.read(&mut buf).await.unwrap();

            let request: Value = serde_json::from_slice(&buf[..n]).unwrap();
            assert_eq!(request["jsonrpc"], "2.0");
            assert_eq!(request["method"], "getversion");
            assert_eq!(request["params"], json!([]));
            assert!(request["id"].is_u64());

            let response = serde_json::json!({
                "jsonrpc": "2.0",
                "result": "1.2.3",
                "id": request["id"],
            });
            let mut resp = serde_json::to_string(&response).unwrap();
            resp.push('\n');
            stream.write_all(resp.as_bytes()).await.unwrap();
        });

        let mut client = GuitarixClient::connect("127.0.0.1", port).await.unwrap();
        let version = client.get_version().await.unwrap();
        assert_eq!(version, "1.2.3");
    }
}
