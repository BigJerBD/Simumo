use crate::components::constant::Identifier;
use crate::components::constant::Rectangle;
use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;
use dim::si::{M, MPS};
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Error};

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
        //let entity = entities.create();

        let json = r#"
            {
                "id": { "val": "salutsalut01" },
                "position": { "x": 32.0, "y": 54.0 },
                "speed": { "val": 2.0 },
                "acceleration": { "val": 0.4 }
            }
        "#;
        let new_car_entities: CarEntity = serde_json::from_str(json).unwrap();
        println!("{:#?}", new_car_entities);

        /*let speed: f64 = rng.gen();
        let px: f64 = rng.gen();
        let py: f64 = rng.gen();
        updater.insert(entity, Identifier("vehicle04".to_string()));
        updater.insert(entity, Speed { val: (4.0 * speed) * MPS });
        updater.insert(entity, Position { x: (20.0 * px) * M, y: (10.0 * py) * M });
        updater.insert(entity, Rectangle { width: 5.0, height: 5.0 });*/
    }
}
