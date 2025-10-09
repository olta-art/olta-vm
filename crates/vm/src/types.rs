use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub type Subscriber = WebSocketStream<TcpStream>;
pub type CollectionName = String;
pub type Collection = BTreeMap<String, Document>;
pub type Collections = BTreeMap<CollectionName, Collection>;

/// collection's transaction analogue
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Document {
    #[serde(rename = "_id")]
    pub id: u64,
    #[serde(rename = "_creator")]
    pub creator: String,
    // #[serde(rename = "_request_id")]
    // pub request_id: String,
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
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Lobby {
    pub process_id: String,
    pub collections: Collections,
    pub processed_txs: HashSet<String>,
    pub hot: bool,
}

/// Delta upates
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct DocumentChanges {
    pub x: Option<String>,
    pub y: Option<String>,
    pub z: Option<String>,
    pub color: Option<String>,
    #[serde(rename = "rotX")]
    pub rot_x: Option<String>,
    #[serde(rename = "rotY")]
    pub rot_y: Option<String>,
    #[serde(rename = "rotZ")]
    pub rot_z: Option<String>,
}
