pub use crate::types::Lobby;
use crate::{
    errors::VMErrors,
    types::{Collection, CollectionData, Collections, Document, DocumentChanges},
};
use std::collections::{BTreeMap, HashSet};

impl Lobby {
    pub fn new(pid: &str) -> Self {
        Self {
            process_id: pid.to_string(),
            collections: BTreeMap::new(),
            processed_txs: HashSet::new(),
            hot: false,
        }
    }

    pub fn get_full_state(&self) -> Result<Collections, VMErrors> {
        Ok(self.collections.clone())
    }

    pub fn get_collection(&self, collection_name: &str) -> Result<Collection, VMErrors> {
        let collection = self
            .collections
            .get(collection_name)
            .ok_or_else(|| VMErrors::CollectionNotFound(("".to_string())))?;
        Ok(collection.clone())
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
        self.hot = true;

        collection.insert(next_id.to_string(), doc);
        Ok(next_id.to_string())
    }
    /// last-writes-win changes
    pub fn update_document(
        &mut self,
        collection_name: &str,
        doc_id: &str,
        changes: DocumentChanges,
    ) -> Result<DocumentChanges, VMErrors> {
        let collection = self
            .collections
            .get_mut(collection_name)
            .ok_or(VMErrors::CollectionNotFound(collection_name.to_string()))?;

        let document =
            collection.get_mut(doc_id).ok_or(VMErrors::DocumentNotFound(doc_id.to_string()))?;

        // Match on the enum to update the appropriate fields
        match &mut document.data {
            CollectionData::Cube(cube) => {
                if let Some(x) = &changes.x {
                    cube.x = x.to_string();
                }
                if let Some(y) = &changes.y {
                    cube.y = y.to_string();
                }
                if let Some(z) = &changes.z {
                    cube.z = z.to_string();
                }
                if let Some(color) = &changes.color {
                    cube.color = color.to_string();
                }
                if let Some(rot_x) = &changes.rot_x {
                    cube.rot_x = rot_x.to_string();
                }
                if let Some(rot_y) = &changes.rot_y {
                    cube.rot_y = rot_y.to_string();
                }
                if let Some(rot_z) = &changes.rot_z {
                    cube.rot_z = rot_z.to_string();
                }
            }
            CollectionData::Vertex(vertex) => {
                if let Some(x) = &changes.x {
                    vertex.x = x.to_string();
                }
                if let Some(y) = &changes.y {
                    vertex.y = y.to_string();
                }
                if let Some(z) = &changes.z {
                    vertex.z = z.to_string();
                }
                if let Some(line_color) = &changes.line_color {
                    vertex.line_color = line_color.to_string();
                }
                if let Some(vertex_color) = &changes.vertex_color {
                    vertex.vertex_color = vertex_color.to_string();
                }
                if let Some(camera_x) = &changes.camera_x {
                    vertex.camera_x = camera_x.to_string();
                }
                if let Some(camera_y) = &changes.camera_y {
                    vertex.camera_y = camera_y.to_string();
                }
                if let Some(camera_z) = &changes.camera_z {
                    vertex.camera_z = camera_z.to_string();
                }
            }
            CollectionData::Splash(splash) => {
                if let Some(x) = &changes.x {
                    splash.x = x.to_string();
                }
                if let Some(y) = &changes.y {
                    splash.y = y.to_string();
                }
                if let Some(seed) = &changes.seed {
                    splash.seed = seed.to_string();
                }
            }
        }

        self.hot = true;

        Ok(changes)
    }

    pub fn delete_document(
        &mut self,
        collection_name: &str,
        document_id: &str,
    ) -> Result<bool, VMErrors> {
        let collection = self
            .collections
            .get_mut(collection_name)
            .ok_or_else(|| VMErrors::CollectionNotFound(format!("{collection_name} not found")))?;
        let res = collection.remove(document_id);

        Ok(res.is_some())
    }
}
