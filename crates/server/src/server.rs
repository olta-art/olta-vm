use crate::types::Subscriber;
use anyhow::{Error, anyhow};
use std::{collections::HashMap, ops::DerefMut};
use vm::Lobby;

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
}
