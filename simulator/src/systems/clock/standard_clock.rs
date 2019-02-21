use crate::ressources::clock;
use crate::systems::clock::ClockSystemType;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct StandardClockSystem;

impl ClockSystemType for StandardClockSystem {}
impl<'a> System<'a> for StandardClockSystem {
    type SystemData = Write<'a, clock::Clock>;

    fn run(&mut self, mut clock: Self::SystemData) {
        clock.update();
    }
}


