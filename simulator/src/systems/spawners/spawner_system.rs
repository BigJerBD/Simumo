use crate::components::constant::Identifier;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;

#[simusystem]
#[derive(Default)]
pub struct SpawnerSystem;

impl<'a> System<'a> for SpawnerSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (clock, entities, updater): Self::SystemData) {
        let entity = entities.create();
        updater.insert(entity, Identifier("vehicle04".to_string()));
    }
}
