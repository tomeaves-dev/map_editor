use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub layer_id: Uuid,
    pub children: Vec<GroupChild>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupChild {
    Group(Group),
    Brush(Uuid),
    Entity(Uuid),
}

impl Group {
    pub fn new(name: impl Into<String>, layer_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            layer_id,
            children: Vec::new(),
        }
    }

    pub fn add_brush(&mut self, brush_id: Uuid) {
        self.children.push(GroupChild::Brush(brush_id));
    }

    pub fn add_entity(&mut self, entity_id: Uuid) {
        self.children.push(GroupChild::Entity(entity_id));
    }

    pub fn add_group(&mut self, group: Group) {
        self.children.push(GroupChild::Group(group));
    }
}