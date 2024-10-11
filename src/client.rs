use crate::common::{Command, Response, KVStoreError};
use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;

pub struct Client {
    stream: TcpStream,
}

impl Client {
    pub async fn connect(addr: &str) -> Result<Self, KVStoreError> {
        let stream = TcpStream::connect(addr).await
            .map_err(|e| KVStoreError::NetworkError(e.to_string()))?;
        Ok(Client { stream })
    }

    pub async fn send_command(&mut self, command: Command) -> Result<Response, KVStoreError> {
        let command_json = serde_json::to_vec(&command)
            .map_err(|e| KVStoreError::SerializationError(e.to_string()))?;

        self.stream.write_all(&command_json).await
            .map_err(|e| KVStoreError::NetworkError(e.to_string()))?;

        let mut buffer = [0; 1024];
        let n = self.stream.read(&mut buffer).await
            .map_err(|e| KVStoreError::NetworkError(e.to_string()))?;

        let response: Response = serde_json::from_slice(&buffer[..n])
            .map_err(|e| KVStoreError::SerializationError(e.to_string()))?;

        Ok(response)
    }

    pub async fn get(&mut self, key: String) -> Result<Response, KVStoreError> {
        self.send_command(Command::Get { key }).await
    }

    pub async fn set(&mut self, key: String, value: String) -> Result<Response, KVStoreError> {
        self.send_command(Command::Set { key, value }).await
    }

    pub async fn delete(&mut self, key: String) -> Result<Response, KVStoreError> {
        self.send_command(Command::Delete { key }).await
    }
}