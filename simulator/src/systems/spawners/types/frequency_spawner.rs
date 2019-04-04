use crate::entities::entity_type::Instantiable;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::ressources::lane_graph::LaneGraph;
use crate::ressources::lane_graph::NodeId;
use crate::ressources::random::Random;
use rand::distributions::{Distribution, Normal};
use rand::Rng;
use simumo_derive::simusystem;
use specs::prelude::{Entities, LazyUpdate, Read, ReadExpect, System, Write};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

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
        Write<'a, Random>,
        Entities<'a>,
        ReadExpect<'a, LaneGraph>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (_clock, mut random, entities, lane_graph, updater): Self::SystemData) {
        let normal_dist = Normal::new(0.015, 0.003);
        let num_cars_to_spawn = random.get_rng().gen_range(self.min, self.max);
        for _ in 1..num_cars_to_spawn {
            let position = self.get_random_start_location(&mut random, &lane_graph);
            debug!("vehicule spawned at : node={} towards={}", (position.0).0, (position.0).1);
            let _destination = self.get_random_end_location(&mut random, &lane_graph);
            let speed = normal_dist.sample(random.get_rng());
            let new_car: CarEntity = CarEntity {
                id: "randomid".to_string(),
                position,
                speed,
                acceleration: 0.0,
            };
            new_car.spawn(&entities, &updater);
        }
    }
}

impl FrequencySpawner {
    pub fn get_random_start_location(
        &self,
        random: &mut Random,
        lane_graph: &LaneGraph,
    ) -> ((NodeId, NodeId), f64) {
        let mut pos_n: usize = random.get_rng().gen_range(0, self.start_locations.len());
        let mut from = self.start_locations[pos_n];
        let mut to = lane_graph.graph.edges(from).last();
        while to.is_none() {
            pos_n = random.get_rng().gen_range(0, self.start_locations.len());
            from = self.start_locations[pos_n];
            to = lane_graph.graph.edges(from).last();
        }
        ((from, to.unwrap().1), 0.0)
    }

    pub fn get_random_end_location(
        &self,
        random: &mut Random,
        lane_graph: &LaneGraph,
    ) -> (f64, f64) {
        let pos_n: usize = random.get_rng().gen_range(0, self.end_locations.len());
        lane_graph
            .intersection(self.end_locations[pos_n])
            .position()
    }
}
