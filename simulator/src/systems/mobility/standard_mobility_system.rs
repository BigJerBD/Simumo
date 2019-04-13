use crate::commons::Percentage;
use crate::components::types::agents::Destination;
use crate::components::types::agents::Itinerary;
use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::ressources::lane_graph::LaneGraph;
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
        WriteStorage<'a, Itinerary>,
        ReadStorage<'a, Destination>,
        ReadStorage<'a, Speed>,
        Read<'a, Clock>,
        ReadExpect<'a, LaneGraph>,
    );

    fn run(&mut self, (mut pos, mut itineraries, destinations, vel, clock, lane_graph): Self::SystemData) {
        for (pos, itinerary, _destination, vel) in (&mut pos, &mut itineraries, &destinations, &vel).join() {
            // TODO: Tweak Curve to not need so many conversions
            let ((from, to), percentage) = pos.val;
            let curve = &lane_graph.lane_between((from, to)).unwrap().curve();
            let mut progress = curve.percentage_to_progress(percentage);
            progress += vel.speed * clock.dt;
            pos.val.1 = progress.percentage();
            println!("{:#?}", progress);
            if progress.percentage() == Percentage::upper() {
                if let Some((from, to)) = itinerary.next() {
                    pos.val = ((from, to), Percentage::lower());
                }
            }
        }
    }
}
