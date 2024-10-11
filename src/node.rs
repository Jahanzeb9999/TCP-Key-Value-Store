use crate::common::{Command, Response, KVStoreError, CommandHandler};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use serde_json;

#[derive(Clone)]
pub struct Node {
    store: Arc<Mutex<HashMap<String, String>>>
}


impl Node {
    fn new() -> Self {
        Node {
            store: Arc::new(Mutex::new(HashMap::new()))
        }
    }

    async fn handle_client(&self, mut stream: TcpStream) -> Result<(), KVStoreError> {
        let mut buffer: [u8; 1024] = [0; 1024];
        loop {
            let n = stream.read(&mut buffer).await.map_err(|e| KVStoreError::NetworkError(e.to_string()))?;

            if n == 0 {break;}

            println!("Received raw data: {:?}", &buffer[..n]);

            let command: Command = serde_json::from_slice(&buffer[..n]).map_err(|e| KVStoreError::SerializationError(e.to_string()))?;

            let response = self.handle_command(command).await?;

            let response_json = serde_json::to_vec(&response)
            .map_err(|e| KVStoreError::SerializationError(e.to_string()))?;
            
            stream.write_all(&response_json).await
            .map_err(|e| KVStoreError::NetworkError(e.to_string()))?;

            stream.flush().await.map_err(|e| KVStoreError::NetworkError(e.to_string()))?;

    }
    Ok(())
    }

}



#[async_trait::async_trait]
impl CommandHandler for Node {
    async fn handle_command(&self, command: Command) -> Result<Response, KVStoreError> {
        let mut store = self.store.lock().await;

       match command {

        Command::Get { key } => {
            Ok(match store.get(&key) {
                Some(value) => Response::Value(value.to_string()),
                None => Response::NotFound,
            })
        }

        Command::Set { key, value } => {
            store.insert(key, value);
            Ok(Response::Ok)
        }

        Command::Delete { key } => {
            store.remove(&key);
            Ok(Response::Ok)
        }
       }
    }
}


pub async fn run(address: &str) -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::new();
    let listener = TcpListener::bind(address).await?;
    println!("Node listening on {}", address);

    loop {
        let (stream, _) = listener.accept().await?;
        let node_clone = node.clone();
        tokio::spawn(async move {
            if let Err(e) = node_clone.handle_client(stream).await {
                eprintln!("Error handling client: {}", e);
            }
        });
    }
}






// Summary of the Flow:
// Client sends data (JSON-encoded) to the server.
// Server receives the data into a buffer.
// Server deserializes the data from the buffer into a Command object using serde_json::from_slice.
// Server processes the command (e.g., fetches a value from the key-value store if it's a Get command).
// Server sends a response back to the client based on the result of processing the command.