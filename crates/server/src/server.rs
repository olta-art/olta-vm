use crate::{
    messages::{Input, Output},
    types::Subscriber,
};
use anyhow::{Error, anyhow};
use std::collections::HashMap;
use vm::{Lobby, types::Document};

#[derive(Debug)]
pub struct Server {
    // process_id -> lobby
    pub lobbies: HashMap<String, Lobby>,
    // process_id -> ws subscribers
    pub subscribers: HashMap<String, Vec<Subscriber>>,
}

impl Server {
    pub fn new() -> Self {
        Self { lobbies: HashMap::new(), subscribers: HashMap::new() }
    }
    pub fn add_subscriber(&mut self, pid: &str, sub: Subscriber) -> &mut Self {
        self.subscribers.entry(pid.to_owned()).or_default().push(sub);
        self
    }
    /// TODO: check persistent storage when lobby isnt in memory
    pub fn get_lobby(&mut self, pid: &str) -> Result<&mut Lobby, Error> {
        if self.lobbies.contains_key(pid) {
            return self
                .lobbies
                .get_mut(pid)
                .ok_or_else(|| anyhow!("err lobby not in memory".to_string()));
        } else {
            Err(anyhow!("lobby not in memory"))
        }
    }
    /// TODO: logic sync with persistent storage
    pub fn create_lobby(&mut self, pid: &str) -> Result<bool, Error> {
        if !self.lobbies.contains_key(pid) {
            let _operation = self.lobbies.insert(pid.to_string(), Lobby::new(pid));
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn broadcast_to_lobby(&self, pid: &str, message: Output) -> Result<(), Error> {
        if let Some(subs) = self.subscribers.get(pid) {
            let msg = serde_json::to_string(&message)
                .map_err(|e| anyhow!("failed to serialize message: {}", e))?;

            for sub in subs {
                if let Err(_e) = sub.send(msg.clone()) {
                    eprintln!("failed to send to subscriber channel");
                }
            }
        }
        Ok(())
    }

    pub async fn create_document(
        &mut self,
        pid: &str,
        collection_name: &str,
        document: Document,
    ) -> Result<String, Error> {
        let lobby = self.get_lobby(pid)?;
        let doc_id =
            lobby.create_document(collection_name, document.clone()).map_err(|_e| anyhow!(""))?;

        // broadcast to all subscribers
        self.broadcast_to_lobby(
            pid,
            Output::DocumentCreated {
                process_id: pid.to_string(),
                collection_name: collection_name.to_string(),
                doc_id: doc_id.clone(),
                document,
            },
        )
        .await?;

        Ok(doc_id)
    }
}
