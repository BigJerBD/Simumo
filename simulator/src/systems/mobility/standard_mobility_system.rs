use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::ressources::Clock;

use simumo_derive::simusystem;
use specs::prelude::{Join, Read, ReadStorage, System, WriteStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simusystem]
pub struct StandardMobilitySystem;
impl<'a> System<'a> for StandardMobilitySystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        ReadStorage<'a, Speed>,
        Read<'a, Clock>,
    );

    fn run(&mut self, (mut pos, vel, clock): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.val.x += vel.val * clock.dt;
            pos.val.y += vel.val * clock.dt;
        }
    }
}
