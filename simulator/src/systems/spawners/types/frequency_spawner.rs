use crate::entities::entity_type::Instantiable;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::ressources::lane_graph::LaneGraph;
use crate::systems::sys_prelude::*;
use crate::util::polar_coordinates_to_cartesian;
extern crate rand;
use rand::Rng;
use dim::si::S;

#[simusystem]
#[derive(Default)]
pub struct FrequencySpawner {
    pub start_locations: Vec<u64>,
    pub end_locations: Vec<u64>,
    pub min: i32,
    pub max: i32,
}
impl<'a> System<'a> for FrequencySpawner {
    type SystemData = (
        Read<'a, clock::Clock>,
        Entities<'a>,
        ReadExpect<'a, LaneGraph>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (_clock, entities, lane_graph, updater): Self::SystemData) {
        let num_cars_to_spawn = rand::thread_rng().gen_range(self.min, self.max); 
        for i in 1..num_cars_to_spawn {
            let position = self.get_random_start_location(&lane_graph);
            let destination = self.get_random_end_location(&lane_graph);
            let new_car: CarEntity = CarEntity {
                id: "randomid".to_string(),
                position,
                speed: 20.0,
                acceleration: 0.0,
            };
            new_car.spawn(&entities, &updater);
        }
    }
}

impl FrequencySpawner {
    pub fn get_random_start_location(&self, lane_graph: &LaneGraph) -> (f64, f64) {
        let pos_n: usize = rand::thread_rng().gen_range(0, self.start_locations.len());
        let position: (f64, f64) = lane_graph.intersection(self.start_locations[pos_n]).position();
        polar_coordinates_to_cartesian(position)
    }
    pub fn get_random_end_location(&self, lane_graph: &LaneGraph) -> (f64, f64) {
        let pos_n: usize = rand::thread_rng().gen_range(0, self.end_locations.len());
        let position: (f64, f64) = lane_graph.intersection(self.end_locations[pos_n]).position();
        polar_coordinates_to_cartesian(position)
    }
}
