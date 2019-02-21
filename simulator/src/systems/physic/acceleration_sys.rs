use crate::components::dynamic::*;
use crate::systems::physic::PhysicSysType;

use crate::systems::sys_prelude::*;

#[simusystem]
pub struct AccelerationSys;
impl PhysicSysType for AccelerationSys{}
impl SystemDefinition for AccelerationSys{}
impl<'a> System<'a> for AccelerationSys {
    type SystemData = (WriteStorage<'a, Speed>, ReadStorage<'a, Acceleration>);

    fn run(&mut self, (mut vel, acc): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            vel.0 += acc.0;
        }
    }
}
