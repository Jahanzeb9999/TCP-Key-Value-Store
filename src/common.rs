use serde::{Deserialize, Serialize};
use std::error::Error;
use std::{fmt};
use async_trait::async_trait;

#[async_trait]
pub trait CommandHandler {
    async fn handle_command(&self, command: Command) -> Result<Response, KVStoreError>;
}



#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Command {
    Get {key: String},
    Set {key: String, value: String},
    Delete {key: String},

}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Value(String),
    Ok,
    NotFound,
    Error(String)

}

#[derive(Debug)]
pub enum KVStoreError {
    NetworkError(String),
    SerializationError(String),
    StorageError(String),
}


impl fmt::Display for KVStoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            KVStoreError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            KVStoreError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            KVStoreError::StorageError(msg) => write!(f, "Storage error: {}", msg),
        }
    }
}

impl Error for KVStoreError {}


// utility functions
pub fn is_valid_key(key: &str) -> bool {
    !key.is_empty() && key.len() <= 256
}


// constants
pub const DEFAULT_COORDINATOR_PORT: u16 = 8080;
pub const DEFAULT_NODE_PORT_RANGE: std::ops::Range<u16> = 8081..8090;

