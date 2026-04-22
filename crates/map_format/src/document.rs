use serde::{Deserialize, Serialize};
use crate::types::{Colour, Vec3};
use crate::layer::Layer;
use crate::brush::Brush;
use crate::entity::Entity;
use crate::group::Group;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapDocument {
    pub metadata: MapMetadata,
    pub environment: Environment,
    pub layers: Vec<Layer>,
    pub brushes: Vec<Brush>,
    pub entities: Vec<Entity>,
    pub groups: Vec<Group>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapMetadata {
    pub name: String,
    pub author: String,
    pub description: String,
    pub version: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub ambient_enabled: bool,
    pub ambient_colour: Colour,
    pub ambient_intensity: f32,
    pub sun_enabled: bool,
    pub sun_direction: Vec3,
    pub sun_colour: Colour,
    pub sun_intensity: f32,
}

impl MapDocument {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            metadata: MapMetadata {
                name: name.into(),
                author: String::new(),
                description: String::new(),
                version: 1,
            },
            environment: Environment::default(),
            layers: Vec::new(),
            brushes: Vec::new(),
            entities: Vec::new(),
            groups: Vec::new(),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self {
            ambient_enabled: true,
            ambient_colour: Colour::white(),
            ambient_intensity: 0.3,
            sun_enabled: true,
            sun_direction: Vec3 { x: -0.5, y: -1.0, z: -0.5 },
            sun_colour: Colour::white(),
            sun_intensity: 1.0,
        }
    }
}