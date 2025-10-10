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
mod utils;
mod ws;

use crate::utils::get_env_var;
use server::Server;
use ws::handle_websocket;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db_url = get_env_var("DATABASE_URL")?;
    let host = get_env_var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port: u16 = get_env_var("PORT").ok().and_then(|s| s.parse().ok()).unwrap_or(8080);
    let bind_addr = format!("{host}:{port}");

    // server state
    let server = Arc::new(Mutex::new(Server::new(&db_url).await?));

    let listener = TcpListener::bind(&bind_addr).await?;
    println!("server listening on ws://{host}:{port}");

    while let Ok((stream, addr)) = listener.accept().await {
        println!("new ws connection from: {addr}");
        let server = server.clone();
        tokio::spawn(handle_connection(stream, server));
    }

    Ok(())
}

async fn handle_connection(stream: TcpStream, server: Arc<Mutex<Server>>) {
    let mut process_id = String::from("default");
    let expected_token = get_env_var("TOKEN").unwrap_or_else(|_| "OLTA".into());

    let callback = |req: &Request, response: Response| {
        let dummy = format!("ws://placeholder{}", req.uri());
        if let Ok(url) = Url::parse(&dummy) {
            let segments: Vec<&str> = url.path_segments().unwrap().collect();

            // route: /ws/:pid
            if segments.len() >= 2 && segments[0] == "ws" {
                process_id = segments[1].to_string();
            }

            // token=? in query
            let token_ok = url
                .query_pairs()
                .find(|(k, _)| k == "token")
                .map(|(_, v)| v)
                .map(|v| v == expected_token.as_str())
                .unwrap_or(false);

            if !token_ok {
                return Err(Response::builder().status(401).body(None).unwrap());
            }
        }

        Ok(response)
    };

    match accept_hdr_async(stream, callback).await {
        Ok(ws_stream) => {
            println!("client joined process: {process_id}");
            handle_websocket(ws_stream, process_id, server).await;
        }
        Err(e) => eprintln!("ws connection error: {e}"),
    }
}
