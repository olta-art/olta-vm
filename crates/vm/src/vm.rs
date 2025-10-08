use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};
use crate::errors::VMErrors;

use crate::types::{Document, DocumentChanges, Lobby, Subscriber};

#[derive(Debug)]
pub struct Olta {
    pub process_id: Option<String>,
    pub subscribers: Option<Vec<Subscriber>>,
    pub lobby: Option<Lobby>,
}

impl Olta {
    pub fn init() -> Self {
        Self { process_id: None, subscribers: None, lobby: None }
    }

    pub fn pid(self, pid: &str) -> Self {
        Self { process_id: Some(pid.to_string()), subscribers: self.subscribers, lobby: self.lobby }
    }

    pub fn subscribers(self, s: Vec<Subscriber>) -> Self {
        let mut subs = self.subscribers.unwrap_or_default();
        subs.extend(s);
        Self { process_id: self.process_id, subscribers: Some(subs) , lobby: self.lobby }
    }

    pub fn build(mut self) -> Result<Self, VMErrors> {
        let _ = self.process_id.clone().ok_or_else(|| VMErrors::ProcessNotFound("missing ao process id".to_string()));
        let now_s = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        let lobby = Lobby {
            process_id: self.process_id.clone().unwrap_or_default(),
            subscribers: self.subscribers.take(), // move subscribers ownership to Lobby
            collections: None,
            last_update: Some(now_s)
        };

        self.lobby = Some(lobby);

        Ok(self)
    }
}