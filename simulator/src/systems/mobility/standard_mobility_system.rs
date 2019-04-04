use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::ressources::Clock;
use crate::ressources::lane_graph::LaneGraph;

use simumo_derive::simusystem;
use specs::prelude::{Join, Read, ReadStorage, ReadExpect, System, WriteStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct StandardMobilitySystem;
impl<'a> System<'a> for StandardMobilitySystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Speed>,
        Read<'a, Clock>,
        ReadExpect<'a, LaneGraph>,
    );

    fn run(&mut self, (mut pos, vel, clock, lane_graph): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            // TODO: Tweak Curve to not need so many converstions
            let (edge, percentage) = pos.val;
            let curve = &lane_graph.lane_between(edge).curve;
            let mut distance = curve.percentage_to_distance(percentage);
            distance += vel.val * clock.dt;
            pos.val.1 = curve.distance_to_percentage(distance);
        }
    }
}
