use serde::{Deserialize, Serialize};
use vm::types::{Collections, Document, DocumentChanges};

#[derive(Serialize, Deserialize)]
pub enum Input {
    JoinProcess { process_id: String },
    CreateDocument { collection_name: String, document: Document },
    UpdateDocument { collection_name: String, doc_id: String, changes: DocumentChanges },
    DeleteDocument { collection_name: String, doc_id: String },
}

#[derive(Serialize, Deserialize)]
pub enum Output {
    FullSync {
        process_id: String,
        collections: Collections,
    },
    DocumentCreated {
        process_id: String,
        collection_name: String,
        doc_id: String,
        document: Document,
    },
    DocumentUpdated {
        process_id: String,
        collection_name: String,
        doc_id: String,
        changes: DocumentChanges,
    },
    DocumentDeleted {
        process_id: String,
        collection_name: String,
        doc_id: String,
    },
    Error {
        message: String,
    },
}
