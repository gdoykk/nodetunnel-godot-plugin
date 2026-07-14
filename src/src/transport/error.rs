use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("Failed to bind UDP socket: {0}")]
    BindError(#[from] std::io::Error),
}