use crate::components::dynamic::*;
use crate::systems::physic::PhysicSystemType;

use crate::ressources::Clock;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct AccelerationSystem;
impl PhysicSystemType for AccelerationSystem {}
impl<'a> System<'a> for AccelerationSystem {
    type SystemData = (
        WriteStorage<'a, Speed>,
        ReadStorage<'a, Acceleration>,
        Read<'a, Clock>,
    );

    fn run(&mut self, (mut vel, acc, clock): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            vel.val += acc.val * clock.dt;
        }
    }
}
