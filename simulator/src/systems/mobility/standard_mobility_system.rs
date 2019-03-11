use crate::components::dynamic::Position;
use crate::components::types::dynamic::Speed;
use crate::ressources::Clock;
use crate::systems::sys_prelude::*;

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
            pos.x += vel.val * clock.dt;
            pos.y += vel.val * clock.dt;
        }
    }
}
