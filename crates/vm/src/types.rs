use serde::{Serialize, Deserialize};
use std::collections::BTreeMap;
use tokio_tungstenite::{WebSocketStream};
use tokio::net::TcpStream;

pub type Subscriber = WebSocketStream<TcpStream>;
/// collection's transaction analogue
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Document {
    #[serde(rename = "_id")]
    pub id: u64,
    #[serde(rename = "_creator")]
    pub creator: String,
    pub x: String,
    pub y: String,
    pub z: String,
    pub color: String,
    #[serde(rename = "rotX")]
    pub rot_x: String,
    #[serde(rename = "rotY")]
    pub rot_y: String,
    #[serde(rename = "rotZ")]
    pub rot_z: String,
}

/// an artwork lobby - instance
#[derive(Debug, Default)]
pub struct Lobby {
    pub process_id: String,
    pub collections: Option<BTreeMap<String, BTreeMap<String, Document>>>,
    pub subscribers: Option<Vec<Subscriber>>,
    pub last_update: Option<u64>,
}

/// Delta upates
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct  DocumentChanges {
    pub x: Option<String>,
    pub y: Option<String>,
    pub z: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "rotX")]
    pub rot_x: Option<String>,
    #[serde(rename = "rotY")]
    pub rot_y: Option<String>,
    #[serde(rename = "rotZ")]
    pub rot_z: Option<String>
}