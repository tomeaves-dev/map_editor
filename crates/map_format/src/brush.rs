use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::types::{Vec2, Vec3};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Brush {
    pub id: Uuid,
    pub planes: Vec<Plane>,
    pub faces: Vec<Face>,
    pub layer_id: Uuid,
    pub group_id: Option<Uuid>,
    pub inverted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Plane {
    pub normal: Vec3,
    pub distance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Face {
    pub texture: String,
    pub scale: Vec2,
    pub offset: Vec2,
    pub rotation: f32,
}

impl Brush {
    pub fn new(layer_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            planes: Vec::new(),
            faces: Vec::new(),
            layer_id,
            group_id: None,
            inverted: false,
        }
    }
}

impl Face {
    pub fn new(texture: impl Into<String>) -> Self {
        Self {
            texture: texture.into(),
            scale: Vec2 { x: 1.0, y: 1.0 },
            offset: Vec2 { x: 0.0, y: 0.0 },
            rotation: 0.0,
        }
    }
}