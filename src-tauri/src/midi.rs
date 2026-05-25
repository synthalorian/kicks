use std::sync::mpsc;

use midir::{MidiInput, MidiInputConnection, MidiInputPort};

/// A raw MIDI CC event captured from an input device.
pub type MidiEvent = (u8, u8, u8); // (channel, controller_number, value 0-127)

/// Manages a connection to a MIDI input device.
///
/// Uses `midir` to open a port and forward CC messages into an mpsc channel.
/// The channel is drained by the `poll_events` Tauri command, which also applies
/// parameter mappings against the engine.
pub struct MidiManager {
    connection: Option<MidiInputConnection<()>>,
    event_rx: mpsc::Receiver<MidiEvent>,
    event_tx: mpsc::Sender<MidiEvent>,
}

impl MidiManager {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            connection: None,
            event_rx: rx,
            event_tx: tx,
        }
    }

    /// List all available MIDI input port names.
    pub fn list_ports() -> Result<Vec<String>, String> {
        let midi_in = MidiInput::new("Kicks").map_err(|e| format!("Failed to create MidiInput: {}", e))?;
        let ports = midi_in.ports();
        let names: Result<Vec<_>, _> = ports.iter().map(|p| midi_in.port_name(p)).collect();
        names.map_err(|e| format!("Failed to get port name: {}", e))
    }

    /// Connect to a MIDI input port by name.
    /// Returns an error if the port is not found or connection fails.
    pub fn connect(&mut self, port_name: &str) -> Result<(), String> {
        // Drop any existing connection first
        self.disconnect();

        let midi_in = MidiInput::new("Kicks").map_err(|e| format!("Failed to create MidiInput: {}", e))?;
        let ports = midi_in.ports();

        let port: MidiInputPort = ports
            .into_iter()
            .find(|p| midi_in.port_name(p).ok().as_deref() == Some(port_name))
            .ok_or_else(|| format!("MIDI port '{}' not found", port_name))?;

        let tx = self.event_tx.clone();
        let conn = midi_in
            .connect(
                &port,
                port_name,
                move |_stamp, message, _| {
                    // Filter for Control Change (CC) messages: status byte 0xB0-0xBF
                    if message.len() >= 3 && message[0] & 0xF0 == 0xB0 {
                        let channel = message[0] & 0x0F;
                        let controller = message[1];
                        let value = message[2];
                        let _ = tx.send((channel, controller, value));
                    }
                },
                (),
            )
            .map_err(|e| format!("Failed to connect MIDI: {}", e))?;

        self.connection = Some(conn);
        tracing::info!("Connected to MIDI device: {}", port_name);
        Ok(())
    }

    /// Disconnect from the active MIDI input device.
    pub fn disconnect(&mut self) {
        if self.connection.is_some() {
            // Dropping the connection closes it gracefully
            self.connection = None;
            tracing::info!("Disconnected from MIDI device");
        }
    }

    /// Check if currently connected to a MIDI device.
    pub fn is_connected(&self) -> bool {
        self.connection.is_some()
    }

    /// Drain all pending MIDI events from the channel.
    pub fn drain_events(&self) -> Vec<MidiEvent> {
        let mut events = Vec::new();
        while let Ok(event) = self.event_rx.try_recv() {
            events.push(event);
        }
        events
    }
}
