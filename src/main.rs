use crate::node::run as run_node;
use crate::client::Client;
use tokio::time::sleep;
use std::time::Duration;

mod common;
mod node;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let address = "127.0.0.1:8080";

    // Start the node in a separate task
    let node_handle = tokio::spawn(async move {
        if let Err(e) = run_node(address).await {
            eprintln!("Node error: {:?}", e);
        }
    });

    // Wait a bit for the node to start up
    sleep(Duration::from_secs(1)).await;

    // Create a client
    let mut client = Client::connect(address).await?;

    // Demonstrate client operations
    println!("Setting key1 to value1");
    let response = client.set("key1".to_string(), "value1".to_string()).await?;
    println!("Set response: {:?}", response);

    println!("Getting key1");
    let response = client.get("key1".to_string()).await?;
    println!("Get response: {:?}", response);

    println!("Deleting key1");
    let response = client.delete("key1".to_string()).await?;
    println!("Delete response: {:?}", response);

    println!("Getting key1 again (should be NotFound)");
    let response = client.get("key1".to_string()).await?;
    println!("Get response: {:?}", response);

    // In a real application, you might want to implement a graceful shutdown mechanism
    // For this example, we'll just let the program end, which will stop the node
    
    println!("Client operations completed. Press Ctrl+C to exit.");
    tokio::signal::ctrl_c().await?;
    println!("Shutting down.");

    // abort the task
    node_handle.abort();

    // wait for the node to finis
    if let Err(e) = node_handle.await {
        eprintln!("Error during node shutdown: {:?}", e);

    }

    Ok(())
}