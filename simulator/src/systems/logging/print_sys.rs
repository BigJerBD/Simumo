use crate::ressources::{clock};
use crate::components::dynamic::*;
use crate::components::statics::*;
use specs::{Entities, Join, Read, ReadStorage, System};

pub struct PrintLog;
impl<'a> System<'a> for PrintLog {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, Time>
    );

    fn run(&mut self, (clock, positions, colors, times): Self::SystemData) {
        println!("Simulation state at {:#?}s", clock.get_time());
        println!("-----------------------------------");
        for pos in positions.join() {
            println!("{:#?}", pos);
        }
        for (color, time) in (&colors, &times).join() {
            println!("{:#?} {:#?}", color, time);
        }
    }
}
