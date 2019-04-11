use std::collections::HashMap;
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

    fn run(&mut self, (_clock, mut random, entities, lane_graph, updater, use_debugger): Self::SystemData) {
        let normal_dist = Normal::new(0.015, 0.003);
        let num_cars_to_spawn = random.get_rng().gen_range(self.min, self.max);
        for _ in 1..num_cars_to_spawn {
            let position = 2557106752; //self.get_random_start_location(&mut random, &lane_graph);
            let destination = 2557106755; //self.get_random_end_location(&mut random, &lane_graph);
            let speed = normal_dist.sample(random.get_rng());
            let path = astar(
                lane_graph.lanes(),
                position,
                |finish| finish == destination,
                |e| lane_graph.get_edge_cost((e.0, e.1)),
                |n| lane_graph.get_estimate_cost_from_node(n, destination),
            );
            println!("{:#?} {:#?} {:#?}", position, destination, path);
            /*let new_car: CarEntity = CarEntity {
                id: "randomid".to_string(),
                position,
                destination,
                speed,
                acceleration: 0.0,
            };
            new_car.spawn(&entities, &updater, use_debugger.0);*/
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

/*fn astar(graph: GraphMap<, start: NodeId, finish: NodeId) {
    let mut closed = Vec::new();
    let mut open = vec![start];
    let mut came_from = HashMap::new();
    let mut node_score = HashMap::new();
    let mut nodes = graph.nodes();
    while let node = nodes.next() {
        println!("{:#?}", node);
    }
    

    // For each node, the cost of getting from the start node to that node.
    gScore := map with default value of Infinity

    // The cost of going from start to start is zero.
    gScore[start] := 0

    // For each node, the total cost of getting from the start node to the goal
    // by passing by that node. That value is partly known, partly heuristic.
    fScore := map with default value of Infinity

    // For the first node, that value is completely heuristic.
    fScore[start] := heuristic_cost_estimate(start, goal)

    while openSet is not empty
        current := the node in openSet having the lowest fScore[] value
        if current = goal
            return reconstruct_path(cameFrom, current)

        openSet.Remove(current)
        closedSet.Add(current)

        for each neighbor of current
            if neighbor in closedSet
                continue		// Ignore the neighbor which is already evaluated.

            // The distance from start to a neighbor
            tentative_gScore := gScore[current] + dist_between(current, neighbor)

            if neighbor not in openSet	// Discover a new node
                openSet.Add(neighbor)
            else if tentative_gScore >= gScore[neighbor]
                continue

            // This path is the best until now. Record it!
            cameFrom[neighbor] := current
            gScore[neighbor] := tentative_gScore
            fScore[neighbor] := gScore[neighbor] + heuristic_cost_estimate(neighbor, goal)
    
}
*/