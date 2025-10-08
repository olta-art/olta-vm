#[derive(Debug)]
pub enum VMErrors {
    ProcessNotFound(String),
    DocumentNotFound(String),
    SerializationError(String),
    WebSocketError(String),
    CollectionUpdateError(String),
    CollectionNotFound(String),
}
