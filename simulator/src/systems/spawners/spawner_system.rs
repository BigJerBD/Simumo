use crate::components::constant::Identifier;
use crate::components::constant::Rectangle;
use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;
use dim::si::{M, MPS};
use rand::prelude::*;

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
        let mut rng = rand::thread_rng();
        let entity = entities.create();
        let speed: f64 = rng.gen();
        let px: f64 = rng.gen();
        let py: f64 = rng.gen();
        updater.insert(entity, Identifier("vehicle04".to_string()));
        updater.insert(entity, Speed { val: (4.0 * speed) * MPS });
        updater.insert(entity, Position { x: (20.0 * px) * M, y: (10.0 * py) * M });
        updater.insert(entity, Rectangle { width: 5.0, height: 5.0 })
    }
}
