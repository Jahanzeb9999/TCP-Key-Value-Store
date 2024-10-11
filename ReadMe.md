# TCP Key-Value Store

A simple TCP-based key-value store implemented in Rust using asynchronous I/O with the **Tokio** runtime. This project demonstrates basic TCP connection handling with a client-server architecture, allowing clients to perform `GET`, `SET`, and `DELETE` operations on a shared key-value store.

## Features
- Asynchronous TCP server using **Tokio**
- Client-server communication using **JSON** serialization
- Simple key-value operations (`GET`, `SET`, `DELETE`)
- Concurrent connection handling
- Graceful shutdown with `Ctrl+C`

## Project Structure
- `client.rs`: Implements a TCP client to connect and send commands to the server.
- `node.rs`: Implements a TCP server (node) to handle commands from clients.
- `common.rs`: Contains shared data structures (commands, responses, errors).
- `main.rs`: Starts the server and demonstrates client interaction.

## Usage

### 1. Run the Server
First, start the TCP server (node). The server listens for incoming connections on `127.0.0.1:8080`.

```bash
cargo run
