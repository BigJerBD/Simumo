use crate::components::dynamic::Position;
use crate::components::types::dynamic::Speed;
use crate::systems::mobility::MobilitySystemType;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct StandardMobilitySystem;
impl MobilitySystemType for StandardMobilitySystem {}
impl<'a> System<'a> for StandardMobilitySystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Speed>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.0;
            pos.y += vel.0;
        }
    }
}
