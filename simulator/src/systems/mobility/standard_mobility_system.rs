use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::ressources::lane_graph::LaneGraph;
use crate::commons::Percentage;
use crate::ressources::Clock;

use simumo_derive::simusystem;
use specs::prelude::{Join, Read, ReadExpect, ReadStorage, System, WriteStorage};
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
            // TODO: Tweak Curve to not need so many conversions
            let ((from, to), percentage) = pos.val;
            let curve = &lane_graph.lane_between((from, to)).unwrap().curve();
            let mut progress = curve.percentage_to_progress(percentage);
            progress += vel.val * clock.dt;
            pos.val.1 = progress.percentage();
            if progress.percentage() == Percentage::upper() {
                if let Some((from, to, _)) = lane_graph.lanes().edges(to).nth(0) {
                    pos.val = ((from, to), Percentage::lower());
                }
            }
        }
    }
}
