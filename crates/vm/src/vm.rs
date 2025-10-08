use crate::{
    errors::VMErrors,
    types::{
        Collection, CollectionName, Collections, Document, DocumentChanges, Lobby, Subscriber,
    },
};
use std::collections::{BTreeMap, HashSet};

impl Lobby {
    pub fn new(pid: &str) -> Self {
        Self {
            process_id: pid.to_string(),
            collections: BTreeMap::new(),
            subscribers: Vec::new(),
            processed_txs: HashSet::new(),
        }
    }

    pub fn get_full_state(&self) -> Result<Collections, VMErrors> {
        Ok(self.collections.clone())
    }

    // server-authoritative design, deterministic sequential documents insertion
    pub fn create_document(
        &mut self,
        collection_name: &str,
        document: Document,
    ) -> Result<String, VMErrors> {
        let collection = self.collections.entry(collection_name.to_string()).or_default();
        // deterministic next sequential id
        let next_id =
            collection.keys().filter_map(|k| k.parse::<u64>().ok()).max().unwrap_or(0) + 1;

        let mut doc = document;
        doc.id = next_id;

        collection.insert(next_id.to_string(), doc);
        Ok(next_id.to_string())
    }
    /// last-writes-win changes
    pub fn update_document(
        &mut self,
        collection_name: &str,
        doc_id: &str,
        changes: DocumentChanges,
    ) -> Result<(), VMErrors> {
        let collection = self
            .collections
            .get_mut(collection_name)
            .ok_or(VMErrors::CollectionNotFound(collection_name.to_string()))?;

        let document =
            collection.get_mut(doc_id).ok_or(VMErrors::DocumentNotFound(doc_id.to_string()))?;

        if let Some(x) = changes.x {
            document.x = x;
        }
        if let Some(y) = changes.y {
            document.y = y;
        }
        if let Some(z) = changes.z {
            document.z = z;
        }
        if let Some(color) = changes.color {
            document.color = color;
        }
        if let Some(rot_x) = changes.rot_x {
            document.rot_x = rot_x;
        }
        if let Some(rot_y) = changes.rot_y {
            document.rot_y = rot_y;
        }
        if let Some(rot_z) = changes.rot_z {
            document.rot_z = rot_z;
        }

        Ok(())
    }
}
