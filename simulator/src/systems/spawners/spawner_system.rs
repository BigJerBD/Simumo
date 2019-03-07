use crate::components::constant::Identifier;
use crate::components::constant::Rectangle;
use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::ressources::entitiesmanagement::EntitiesManager;
use crate::systems::sys_prelude::*;
use crate::entities::entity_type::EntityType;
use crate::entities::entity_type::Instantiable;
use rand::prelude::*;
use std::fs::File;

#[simusystem]
#[derive(Default)]
pub struct SpawnerSystem;

impl<'a> System<'a> for SpawnerSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        Write<'a, EntitiesManager>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (clock, mut entity_manager, entities, updater): Self::SystemData) {
        //entity_manager.spawn(EntityType::CarEntity())

        //new_car_entity.spawn(&entities, updater);
    }
}
