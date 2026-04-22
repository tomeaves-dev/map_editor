use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layer {
    pub id: Uuid,
    pub name: String,
    pub layer_type: LayerType,
    pub visible: bool,
    pub locked: bool,
    pub children: Vec<Layer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LayerType {
    Brush { is_collision: bool },
    Entity,
}

impl Layer {
    pub fn new_brush(name: impl Into<String>, is_collision: bool) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            layer_type: LayerType::Brush { is_collision },
            visible: true,
            locked: false,
            children: Vec::new(),
        }
    }

    pub fn new_entity(name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            layer_type: LayerType::Entity,
            visible: true,
            locked: false,
            children: Vec::new(),
        }
    }
}