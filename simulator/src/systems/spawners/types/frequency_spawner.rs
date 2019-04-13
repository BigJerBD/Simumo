use crate::entities::entity_type::Instantiable;
use crate::entities::types::CarEntity;
use crate::ressources::clock;
use crate::ressources::lane_graph::LaneGraph;
use crate::ressources::lane_graph::NodeId;
use crate::ressources::random::Random;
use crate::simulation::UseDebugger;
use petgraph::algo::astar;
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
        Read<'a, UseDebugger>,
    );

    fn run(
        &mut self,
        (_clock, mut random, entities, lane_graph, updater, use_debugger): Self::SystemData,
    ) {
        let normal_dist = Normal::new(0.015, 0.003);
        let num_cars_to_spawn = random.get_rng().gen_range(self.min, self.max);
        for _ in 1..num_cars_to_spawn {
            let start_node = self.get_random_start_location(&mut random, &lane_graph); //2557106752
            let end_node = self.get_random_end_location(&mut random, &lane_graph); //2557106755
            let speed = normal_dist.sample(random.get_rng());
            if let Some((_cost, path)) = astar(
                    lane_graph.lanes(),
                    start_node,
                    |finish| finish == end_node,
                    |e| lane_graph.get_edge_cost((e.0, e.1)),
                    |n| lane_graph.get_estimate_cost_from_node(n, end_node),
                ) {
                let num_nodes = path.len();
                let new_car: CarEntity = CarEntity {
                    id: "randomid".to_string(),
                    position: ((path[0], path[1]), 0.0),
                    destination: ((path[num_nodes - 2], path[num_nodes - 1]), 1.0),
                    speed,
                    acceleration: 0.0,
                };
                new_car.spawn(&entities, &updater, use_debugger.0);
            } else {
                println!("could not find path from node {:#?} to node {:#?}", start_node, end_node);
            }
        }
    }
}

impl FrequencySpawner {
    pub fn get_random_start_location(
        &self,
        random: &mut Random,
        lane_graph: &LaneGraph,
    ) -> NodeId {
        let mut pos_n: usize = random.get_rng().gen_range(0, self.start_locations.len());
        let mut from = self.start_locations[pos_n];
        /*let mut to = lane_graph.graph.edges(from).last();
        while to.is_none() {
            pos_n = random.get_rng().gen_range(0, self.start_locations.len());
            from = self.start_locations[pos_n];
            to = lane_graph.graph.edges(from).last();
        }
        ((from, to.unwrap().1), 0.0)*/
        from
    }

    pub fn get_random_end_location(
        &self,
        random: &mut Random,
        lane_graph: &LaneGraph,
    ) -> NodeId {
        let pos_n: usize = random.get_rng().gen_range(0, self.end_locations.len());
        self.end_locations[pos_n]
    }
}
