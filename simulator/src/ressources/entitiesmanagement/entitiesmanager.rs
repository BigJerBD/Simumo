use std::collections::HashMap;
use crate::entities::entity_type::EntityType;

#[derive(Default)]
pub struct EntitiesManager {
    pub entities_to_spawn: Vec<EntityType>
}

impl EntitiesManager {
    pub fn new() -> Self {
        Self {
            entities_to_spawn: Vec::new()
        }
    }

    pub fn spawn(&mut self, entity: EntityType) {
        self.entities_to_spawn.push(entity);
    }

    pub fn get_entities_to_spawn(&self) {

    }
    
    pub fn get_entities_to_delete(&self) {
        
    }
}