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
        let json = r#"
            [
                {
                    "id": { "val": "vehicle01" },
                    "type": "vehicle",
                    "position": { "x": 0.0, "y": 0.0 },
                    "speed": { "val": 2.0 }
                },
                {
                    "id": { "val": "vehicle02" },
                    "type": "vehicle",
                    "position": { "x": 0.0, "y": 0.0 },
                    "speed": { "val": 4.0 },
                    "acceleration": { "val": 0.4 }
                },
                {
                    "id": { "val": "vehicle03" },
                    "type": "vehicle",
                    "position": { "x": 0.0, "y": 0.0 },
                    "speed": { "val": 1.5 }
                },
                {
                    "id": { "val": "trafficlight01" },
                    "type": "trafficlight",
                    "light": {
                        "initial_color": "GREEN",
                        "max_green_time": 5.0,
                        "max_yellow_time": 1.5,
                        "time": 3.5
                    }
                },
                {
                    "id": { "val": "trafficlight02" },
                    "type": "trafficlight",
                    "light": {
                        "initial_color": "RED",
                        "max_green_time": 3.0,
                        "max_yellow_time": 1.0,
                        "time": 0.0
                    }
                }
            ]
        "#;
        //entity_manager.spawn(EntityType::CarEntity())

        let new_entities: Vec<EntityType> = serde_json::from_str(json).unwrap();
        println!("{:#?}", new_entities);

        /*let json_file = File::open(Path::new("configs.json")).expect("file not found");
        let entities: Vec<EntityType> =
            serde_json::from_reader(json_file).expect("error while reading json");
        println!("{:#?}", entities);*/

        //new_car_entity.spawn(&entities, updater);
    }
}
