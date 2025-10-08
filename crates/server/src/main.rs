use std::sync::Arc;
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Mutex,
};
use tokio_tungstenite::{
    accept_hdr_async,
    tungstenite::handshake::server::{Request, Response},
};
use url::Url;

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
    let mut process_id = "default".to_string();

    let callback = |req: &Request, response: Response| {
        if let Ok(url) = Url::parse(&format!("http://localhost{}", req.uri().path())) {
            let segments: Vec<&str> = url.path_segments().unwrap().collect();
            // rute path localhost/ws/:pid
            if segments.len() >= 2 && segments[0] == "ws" {
                process_id = segments[1].to_string();
            }
        }
        Ok(response)
    };

    match accept_hdr_async(stream, callback).await {
        Ok(ws_stream) => {
            println!("client joined artwork: {}", process_id);
            handle_websocket(ws_stream, process_id, server).await;
        }
        Err(e) => println!("ws connection error: {}", e),
    }
}
