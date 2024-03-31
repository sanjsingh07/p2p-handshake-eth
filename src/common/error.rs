use reth_ecies::ECIESError;
use reth_eth_wire::errors::P2PStreamError;
use std::fmt::Display;
use tokio::task::JoinError;

#[derive(thiserror::Error, Debug)]
pub enum P2PError {
    #[error("P2P handshake error: {0}")]
    P2PHandshakeError(P2PHandshakeErr),
    #[error("ECIES error make a retry again: {0}")]
    ECIESError(#[from] ECIESError),
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
    #[error("Tokio elapsed error: {0}")]
    TokioElapsedError(#[from] tokio::time::error::Elapsed),
    #[error("P2P stream error: {0}")]
    P2PStreamError(#[from] P2PStreamError),
    #[error("Tokio JoinError error: {0}")]
    TokioJoinError(#[from] JoinError),
}

#[derive(Debug)]
pub struct P2PHandshakeErr {
    message: String,
    address: String,
}

impl P2PHandshakeErr {
    pub fn new(err: P2PError, address: String) -> Self {
        Self {
            message: err.to_string(),
            address,
        }
    }
}

impl Display for P2PHandshakeErr {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[failed] [{}] error: {}", self.address, self.message)
    }
}
