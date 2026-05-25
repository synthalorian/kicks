use crate::client::GuitarixClient;
use crate::error::{Error, Result};
use std::time::Duration;
use tokio::time::sleep;

/// The state of a Guitarix engine connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    Disconnected,
    Connecting,
    Connected,
    Reconnecting,
    Failed,
}

/// Manages a Guitarix engine connection lifecycle with automatic reconnection.
pub struct ConnectionManager {
    host: String,
    port: u16,
    state: ConnectionState,
    retry_count: u32,
    max_retries: u32,
    /// Multiplier for exponential backoff: delay = base_delay * 2^retry
    base_delay_ms: u64,
    /// Maximum backoff delay cap
    max_delay_ms: u64,
}

impl ConnectionManager {
    pub fn new(host: &str, port: u16) -> Self {
        Self {
            host: host.to_string(),
            port,
            state: ConnectionState::Disconnected,
            retry_count: 0,
            max_retries: 10,
            base_delay_ms: 200,
            max_delay_ms: 10_000,
        }
    }

    pub fn state(&self) -> ConnectionState {
        self.state
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn set_host_port(&mut self, host: &str, port: u16) {
        self.host = host.to_string();
        self.port = port;
    }

    /// Attempt to connect once. Returns the client on success.
    pub async fn connect_once(&mut self) -> Result<GuitarixClient> {
        self.state = ConnectionState::Connecting;
        match GuitarixClient::connect(&self.host, self.port).await {
            Ok(client) => {
                self.state = ConnectionState::Connected;
                self.retry_count = 0;
                tracing::info!("Connected to Guitarix at {}:{}", self.host, self.port);
                Ok(client)
            }
            Err(e) => {
                self.state = ConnectionState::Failed;
                tracing::warn!("Failed to connect to Guitarix at {}:{}: {}", self.host, self.port, e);
                Err(e)
            }
        }
    }

    /// Connect with exponential backoff retries. Returns the client on first success,
    /// or the last error after exhausting retries.
    pub async fn connect_with_retry(&mut self) -> Result<GuitarixClient> {
        let mut last_error = Error::ConnectionClosed;

        for attempt in 0..=self.max_retries {
            if attempt > 0 {
                let delay = self.backoff_delay(attempt);
                tracing::info!(
                    "Reconnect attempt {}/{} in {:?}...",
                    attempt, self.max_retries, delay
                );
                self.state = ConnectionState::Reconnecting;
                sleep(delay).await;
            }

            match self.connect_once().await {
                Ok(client) => return Ok(client),
                Err(e) => {
                    last_error = e;
                }
            }
        }

        self.state = ConnectionState::Failed;
        tracing::error!(
            "Failed to connect after {} retries ({}:{}): {}",
            self.max_retries, self.host, self.port, last_error
        );
        Err(last_error)
    }

    /// Reset retry count (e.g., after a successful manual reconnect).
    pub fn reset_retries(&mut self) {
        self.retry_count = 0;
    }

    /// Compute exponential backoff delay capped at max_delay_ms.
    fn backoff_delay(&self, attempt: u32) -> Duration {
        let delay = self.base_delay_ms * 2u64.saturating_pow(attempt.saturating_sub(1));
        Duration::from_millis(delay.min(self.max_delay_ms))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_state_disconnected() {
        let mgr = ConnectionManager::new("127.0.0.1", 4040);
        assert_eq!(mgr.state(), ConnectionState::Disconnected);
    }

    #[test]
    fn test_host_port() {
        let mgr = ConnectionManager::new("localhost", 8080);
        assert_eq!(mgr.host(), "localhost");
        assert_eq!(mgr.port(), 8080);
    }

    #[test]
    fn test_backoff_delay_ramps_up() {
        let mgr = ConnectionManager::new("127.0.0.1", 4040);
        let d1 = mgr.backoff_delay(1);
        let d2 = mgr.backoff_delay(2);
        let d3 = mgr.backoff_delay(3);
        // attempt 1: base = 200ms
        // attempt 2: 200 * 2^1 = 400ms
        // attempt 3: 200 * 2^2 = 800ms
        assert_eq!(d1, Duration::from_millis(200));
        assert_eq!(d2, Duration::from_millis(400));
        assert_eq!(d3, Duration::from_millis(800));
    }

    #[test]
    fn test_backoff_delay_capped() {
        let mut mgr = ConnectionManager::new("127.0.0.1", 4040);
        mgr.max_delay_ms = 1000;
        // attempt 10: 200 * 2^9 = 102400ms, capped at 1000ms
        let d = mgr.backoff_delay(10);
        assert_eq!(d, Duration::from_millis(1000));
    }

    #[test]
    fn test_set_host_port() {
        let mut mgr = ConnectionManager::new("old", 1);
        mgr.set_host_port("new", 9999);
        assert_eq!(mgr.host(), "new");
        assert_eq!(mgr.port(), 9999);
    }
}
