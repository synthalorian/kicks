// Kicks — Guitarix JSON-RPC Client Library
//
// This crate provides a Rust client for Guitarix's JSON-RPC 2.0 protocol.
// It enables remote control of a running Guitarix engine (headless or GUI mode)
// via TCP, covering presets, banks, parameters, plugins, MIDI, tuner, and more.

pub mod client;
pub mod connection;
pub mod error;
pub mod helpers;
pub mod launcher;
pub mod types;

pub use client::GuitarixClient;
pub use connection::{ConnectionManager, ConnectionState};
pub use error::Error;
pub use launcher::GuitarixProcess;
