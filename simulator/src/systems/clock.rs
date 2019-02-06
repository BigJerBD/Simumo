use crate::ressources::clock;
use specs::Read;
use specs::System;
use specs::Write;

pub struct ClockSys;
impl<'a> System<'a> for ClockSys {
    type SystemData = Write<'a, clock::Clock>;

    fn run(&mut self, mut clock: Self::SystemData) {
        clock.update();
    }
}
