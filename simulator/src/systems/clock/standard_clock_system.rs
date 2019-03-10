use crate::ressources;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct StandardClockSystem;
impl<'a> System<'a> for StandardClockSystem {
    type SystemData = Write<'a, ressources::Clock>;

    fn run(&mut self, mut clock: Self::SystemData) {
        clock.update();
    }
}
