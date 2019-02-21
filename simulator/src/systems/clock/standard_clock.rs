use crate::ressources::clock;
use crate::systems::clock::ClockSysType;
use crate::systems::sys_prelude::*;

#[simusystem]
pub struct StandardClockSys;

impl ClockSysType for StandardClockSys {}
impl SystemDefinition for StandardClockSys{}
impl<'a> System<'a> for StandardClockSys {
    type SystemData = Write<'a, clock::Clock>;

    fn run(&mut self, mut clock: Self::SystemData) {
        clock.update();
    }
}


