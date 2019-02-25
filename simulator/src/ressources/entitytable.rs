use std::collections::HashMap;
use specs::prelude::{Entity};

#[derive(Default)]
pub struct EntityTable {
    table: HashMap<String, Entity>
}

impl EntityTable {
    pub fn new() -> Self {
        Self {
            table: HashMap::new()
        }
    }
    pub fn insert(&mut self, id: String, entity: Entity) {
        self.table.insert(id, entity);
    }
    pub fn get_entity(&self, id: &str) -> Option<&Entity> {
        match self.table.get(id) {
            Some(entity) => Some(&entity),
            None => None
        }
    }
}