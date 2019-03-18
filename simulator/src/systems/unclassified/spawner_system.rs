use crate::entities::entity_type::Instantiable;
use crate::entities::types::CarEntity;
use crate::ressources::clock;

use specs::prelude::{Entities, LazyUpdate, Read, System};
use simumo_derive::simusystem;
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
#[derive(Default)]
pub struct SpawnerSystem;

impl<'a> System<'a> for SpawnerSystem {
    type SystemData = (Read<'a, clock::Clock>, Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (_clock, entities, updater): Self::SystemData) {
        //todo note this could be an component added in the simulation
        let json = r#"
            {
                "id": "spawnedvehicle",
                "type": "vehicle",
                "position": {
                    "x": 24.0,
                    "y": 32.0
                },
                "speed": {
                    "val": 6.0
                },
                "acceleration": {
                    "val": 0.5
                }
            }"#;
        let new_car: CarEntity = serde_json::from_str(json).unwrap();
        new_car.spawn(&entities, updater);
    }
}
