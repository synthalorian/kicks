use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Connection closed by server")]
    ConnectionClosed,

    #[error("Timeout waiting for response")]
    Timeout,
}
