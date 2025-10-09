use crate::{
    messages::{Input, Output},
    server::Server,
};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{Mutex, mpsc},
};
use tokio_tungstenite::{WebSocketStream, tungstenite::Message};

pub async fn handle_websocket(
    ws_stream: WebSocketStream<TcpStream>,
    process_id: String,
    server: Arc<Mutex<Server>>,
) {
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    {
        let mut server = server.lock().await;
        server.create_lobby(&process_id).ok();
        server.add_subscriber(&process_id, tx);
    }

    {
        let mut server = server.lock().await;
        if let Ok(lobby) = server.get_lobby(&process_id) {
            if let Ok(collections) = lobby.get_full_state() {
                let full_sync = Output::FullSync { process_id: process_id.clone(), collections };
                if let Ok(msg) = serde_json::to_string(&full_sync) {
                    let _ = ws_sender.send(Message::Text(msg.into())).await;
                }
            }
        }
    }

    // fwd channel messages to ws
    let ws_sender_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if ws_sender.send(Message::Text(message.into())).await.is_err() {
                break;
            }
        }
    });

    // msgs ingress handlers
    while let Some(message) = ws_receiver.next().await {
        if let Ok(Message::Text(text)) = message {
            if let Ok(input) = serde_json::from_str::<Input>(&text) {
                let mut server = server.lock().await;

                match input {
                    Input::CreateDocument { collection_name, document } => {
                        match server.create_document(&process_id, &collection_name, document).await
                        {
                            Ok(doc_id) => {
                                println!(
                                    "created document {} in collection {}",
                                    doc_id, collection_name
                                );
                            }
                            Err(e) => {
                                eprintln!("failed to create document: {}", e);
                            }
                        }
                    }

                    Input::UpdateDocument { collection_name, doc_id, changes } => {
                        match server
                            .update_document(&process_id, &collection_name, &doc_id, changes)
                            .await
                        {
                            Ok(_) => {
                                println!(
                                    "updated document {} in collection {}",
                                    doc_id, collection_name
                                );
                            }
                            Err(e) => {
                                eprintln!("failed to update document: {}", e);
                            }
                        }
                    }

                    Input::DeleteDocument { collection_name, doc_id } => {
                        match server.delete_document(&process_id, &collection_name, &doc_id).await {
                            Ok(_) => {
                                println!(
                                    "deleted document {} from collection {}",
                                    doc_id, collection_name
                                );
                            }
                            Err(e) => {
                                eprintln!("failed to delete document: {}", e);
                            }
                        }
                    }
                    Input::JoinProcess { .. } => todo!(),
                }
            } else {
                eprintln!("failed to parse message: {}", text);
            }
        } else if let Ok(Message::Close(_)) = message {
            println!("ws connection closed for process {}", process_id);
            break;
        }
    }

    ws_sender_task.abort();
}
