use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;

mod messages;
mod server;
mod types;
mod ws;

use server::Server;
use ws::handle_websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = Arc::new(Mutex::new(Server::new()));
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("server listening on ws://127.0.0.1:8080");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("new ws connection from: {}", addr);
        let server = server.clone();
        tokio::spawn(handle_connection(stream, server));
    }
    Ok(())
}

async fn handle_connection(stream: TcpStream, server: Arc<Mutex<Server>>) {
    match accept_async(stream).await {
        Ok(ws_stream) => {
            // dummy test
            let process_id = "test123".to_string();
            handle_websocket(ws_stream, process_id, server).await;
        }
        Err(e) => println!("ws connection error: {}", e),
    }
}
