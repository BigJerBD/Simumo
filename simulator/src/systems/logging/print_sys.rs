use crate::ressources::{clock};
use crate::components::dynamic::*;
use crate::components::statics::*;
use specs::{Join, Read, ReadStorage, System};

pub struct PrintLog;
impl<'a> System<'a> for PrintLog {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Color>,
        ReadStorage<'a, GreenTime>
    );

    fn run(&mut self, (clock, positions, colors, green_times): Self::SystemData) {
        println!("Simulation state at {:#?}s", clock.get_time());
        println!("-----------------------------------");
        for pos in positions.join() {
            println!("{:#?}", pos);
        }
        for (color, green_time) in (&colors, &green_times).join() {
            println!("{:#?} {:#?}", color, green_time);
        }
    }
}
