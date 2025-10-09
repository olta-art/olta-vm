/// TODO: proper error types
use crate::{
    messages::{Input, Output},
    types::Subscriber,
};
use anyhow::{Error, anyhow};
use tokio::sync::mpsc;
use std::collections::HashMap;
use vm::{
    Lobby,
    types::{Document, DocumentChanges},
};
use storage::{Database, DbOperation, DatabaseWorker};

#[derive(Debug)]
pub struct Server {
    // process_id -> lobby
    pub lobbies: HashMap<String, Lobby>,
    // process_id -> ws subscribers
    pub subscribers: HashMap<String, Vec<Subscriber>>,
    // Background queue
    db_sender: mpsc::UnboundedSender<DbOperation>,
    // persistent storage
    storage: Database,    
}

impl Server {
    pub async fn new(database_url: &str) -> Result<Self, Error> {
        let database = Database::new(database_url).await?;
        database.run_migrations().await?;
        
        // Crcreacreateteeate background worker channel
        let (db_sender, db_receiver) = mpsc::unbounded_channel::<DbOperation>();
        
        // start db worker
        let db_worker = DatabaseWorker::new(database.clone(), db_receiver);
        tokio::spawn(async move {
            db_worker.run().await;
        });
        
        Ok(Self {
            lobbies: HashMap::new(),
            subscribers: HashMap::new(),
            storage: database,
            db_sender,
        })
    }
    pub fn add_subscriber(&mut self, pid: &str, sub: Subscriber) -> &mut Self {
        self.subscribers.entry(pid.to_owned()).or_default().push(sub);
        self
    }
    /// get lobby from memory if hot or load from database if not
pub async fn get_lobby(&mut self, pid: &str) -> Result<&mut Lobby, Error> {
    if self.lobbies.contains_key(pid) {
        
        return self.lobbies.get_mut(pid)
            .ok_or_else(|| anyhow!("lobby not in memory"));
    }
    
    // If lobby not in memory - load from storage
    match self.storage.load_process_state(pid).await {
        Ok(Some(state_json)) => {
            
            let lobby: Lobby = serde_json::from_str(&state_json)
                .map_err(|e| anyhow!("failed to deserialize lobby: {}", e))?;
            
            // match self.database.get_process_documents(pid).await {
            //     Ok(documents) => {
            //         println!("Rebuilding collections from {} documents", documents.len());
                    
            //         // Group documents by collection
            //         for (collection_name, doc_id, _process_id, document_data) in documents {
            //             // Deserialize the document
            //             let document: vm::types::Document = serde_json::from_value(document_data)
            //                 .map_err(|e| anyhow!("Failed to deserialize document: {}", e))?;
                        
            //             // Get or create collection
            //             let collection = lobby.collections
            //                 .entry(collection_name)
            //                 .or_insert_with(|| std::collections::BTreeMap::new());
                        
            //             // Insert document with doc_id as key
            //             collection.insert(doc_id, document);
            //         }
                    
            //         println!("Rebuilt lobby with {} collections", lobby.collections.len());
            //         for (cname, coll) in &lobby.collections {
            //             println!("  Collection '{}': {} documents", cname, coll.len());
            //         }
            //     }
            //     Err(e) => {
            //         println!("Warning: Failed to load documents for lobby {}: {}", pid, e);
            //         // Continue with empty collections - better than failing completely
            //     }
            // }
            
            // insert rebuilt lobby into memory (make hot)
            self.lobbies.insert(pid.to_string(), lobby);
            
            
            self.lobbies.get_mut(pid)
                .ok_or_else(|| anyhow!("Failed to insert lobby"))
        }
        Ok(None) => {
            // new lobby - create in memory and storage
            println!("creating new lobby: {}", pid);
            
            let lobby = Lobby::new(pid);
            self.lobbies.insert(pid.to_string(), lobby);
            
            // save to db in background
            let lobby_json = serde_json::to_string(self.lobbies.get(pid).unwrap())?;
            let _ = self.db_sender.send(DbOperation::SaveProcessState {
                process_id: pid.to_string(),
                full_state: lobby_json,
                is_hot: true,
            });
            
            self.lobbies.get_mut(pid)
                .ok_or_else(|| anyhow!("failed to get new lobby"))
        }
        Err(e) => Err(anyhow!("failed to load lobby from storage: {}", e))
    }
}

    
    pub async fn create_lobby(&mut self, pid: &str) -> Result<bool, Error> {
        if !self.lobbies.contains_key(pid) {
            let lobby = Lobby::new(pid);
            self.lobbies.insert(pid.to_string(), lobby);
            
            let lobby_json = serde_json::to_string(self.lobbies.get(pid).unwrap())?;
            let _ = self.db_sender.send(DbOperation::SaveProcessState {
                process_id: pid.to_string(),
                full_state: lobby_json,
                is_hot: true,
            });
            
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
    // do all work that needs &mut Lobby without awaiting
    let (doc_id, complete_state) = {
        let lobby = self.get_lobby(pid).await?; 
        let doc_id = lobby
            .create_document(collection_name, document.clone())
            .map_err(|_e| anyhow!("create_document failed"))?;

        let complete_state = serde_json::to_string(&*lobby)?;
        (doc_id, complete_state)
    }; 


    self.broadcast_to_lobby(
        pid,
        Output::DocumentCreated {
            process_id: pid.to_string(),
            collection_name: collection_name.to_string(),
            doc_id: doc_id.clone(),
            document: document.clone(),
        },
    )
    .await?;
    self.storage.save_process_state(&pid, &complete_state, true).await?;

    Ok(doc_id)
}


    pub async fn update_document(
        &mut self,
        pid: &str,
        collection_name: &str,
        doc_id: &str,
        changes: DocumentChanges,
    ) -> Result<(), Error> {
        let lobby = self.get_lobby(pid).await?;
        let res = lobby
            .update_document(collection_name, doc_id, changes)
            .map_err(|_| anyhow!("".to_string()))?;

        let _ = self.broadcast_to_lobby(
            pid,
            Output::DocumentUpdated {
                process_id: pid.to_string(),
                collection_name: collection_name.to_string(),
                doc_id: doc_id.to_string(),
                changes: res,
            },
        );
        Ok(())
    }

    pub async fn delete_document(
        &mut self,
        pid: &str,
        collection_name: &str,
        doc_id: &str,
    ) -> Result<(), Error> {
        let lobby = self.get_lobby(pid).await?;
        let success =
            lobby.delete_document(collection_name, doc_id).map_err(|_| anyhow!("".to_string()))?;

        if success {
            let _ = self.broadcast_to_lobby(
                pid,
                Output::DocumentDeleted {
                    process_id: pid.to_string(),
                    collection_name: collection_name.to_string(),
                    doc_id: doc_id.to_string(),
                },
            );
            Ok(())
        } else {
            Err(anyhow!("".to_string()))
        }
    }
}
