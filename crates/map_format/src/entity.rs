use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;
use crate::types::Vec3;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub class: String,
    pub position: Vec3,
    pub properties: HashMap<String, String>,
    pub layer_id: Uuid,
    pub group_id: Option<Uuid>,
}

impl Entity {
    pub fn new(class: impl Into<String>, position: Vec3, layer_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            class: class.into(),
            position,
            properties: HashMap::new(),
            layer_id,
            group_id: None,
        }
    }

    pub fn set_property(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.properties.insert(key.into(), value.into());
    }

    pub fn get_property(&self, key: &str) -> Option<&String> {
        self.properties.get(key)
    }
}