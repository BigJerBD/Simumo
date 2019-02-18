use crate::components::dynamic::Position;
use crate::components::types::dynamic::Speed;
use crate::systems::mobility::MobilitySys;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct StandardMobilitySys;
impl  MobilitySys for StandardMobilitySys{}
impl<'a> System<'a> for StandardMobilitySys {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Speed>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.0;
            pos.y += vel.0;
        }
    }
}
