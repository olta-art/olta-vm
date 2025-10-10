use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashSet};
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;

pub type Subscriber = WebSocketStream<TcpStream>;
pub type CollectionName = String;
pub type Collection = BTreeMap<String, Document>;
pub type Collections = BTreeMap<CollectionName, Collection>;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Vertex {
    pub x: String,
    pub y: String,
    pub z: String,
    #[serde(rename = "lineColor")]
    pub line_color: String,
    #[serde(rename = "vertexColor")]
    pub vertex_color: String,
    #[serde(rename = "cameraX")]
    pub camera_x: String,
    #[serde(rename = "cameraY")]
    pub camera_y: String,
    #[serde(rename = "cameraZ")]
    pub camera_z: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Cube {
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Splash {
    pub x: String,
    pub y: String,
    pub seed: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(tag = "type")]
pub enum CollectionData {
    #[serde(rename = "cubes")]
    Cube(Cube),
    #[serde(rename = "vertices")]
    Vertex(Vertex),
    #[serde(rename = "splashes")]
    Splash(Splash),
}

/// collection's transaction analogue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Document {
    // generic fields
    #[serde(rename = "_id")]
    pub id: u64,
    #[serde(rename = "_creator")]
    pub creator: String,
    pub request_id: Option<String>,
    // specific collection's type data
    #[serde(flatten)]
    pub data: CollectionData,
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
    // common
    pub x: Option<String>,
    pub y: Option<String>,
    pub z: Option<String>,
    // cube
    pub color: Option<String>,
    #[serde(rename = "rotX")]
    pub rot_x: Option<String>,
    #[serde(rename = "rotY")]
    pub rot_y: Option<String>,
    #[serde(rename = "rotZ")]
    pub rot_z: Option<String>,
    // vertex
    #[serde(rename = "lineColor")]
    pub line_color: Option<String>,
    #[serde(rename = "vertexColor")]
    pub vertex_color: Option<String>,
    #[serde(rename = "cameraX")]
    pub camera_x: Option<String>,
    #[serde(rename = "cameraY")]
    pub camera_y: Option<String>,
    #[serde(rename = "cameraZ")]
    pub camera_z: Option<String>,
    // splash
    pub seed: Option<String>,
}
