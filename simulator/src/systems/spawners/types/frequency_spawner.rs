use crate::components::spawner::SpawnerType;
use crate::entities::entity_type::Instantiable;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;

#[simusystem]
#[derive(Default)]
pub struct FrequencySpawner {
    pub locations: Vec<i32>
}
impl<'a> System<'a> for FrequencySpawner {
    type SystemData = (
        Read<'a, clock::Clock>,
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, SpawnerType>,
    );

    fn run(&mut self, (_clock, entities, updater, spawner_types): Self::SystemData) {
        for (entity, spawner_type) in (&entities, &spawner_types).join() {
            println!("{:#?}", spawner_type);
        }
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
                },
                "spawner": {
                    "type": "Frequency",
                    "period": 1
                }
            }"#;
        let new_car: CarEntity = serde_json::from_str(json).unwrap();
        new_car.spawn(&entities, updater);
    }
}
