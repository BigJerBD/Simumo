use crate::ressources::{clock};
use crate::components::dynamic::*;
use crate::components::statics::trafficlight::*;
use specs::{Entities, Join, Read, ReadStorage, System};

pub struct PrintLog;
impl<'a> System<'a> for PrintLog {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Light>
    );

    fn run(&mut self, (clock, positions, lights): Self::SystemData) {
        println!("Simulation state at {:#?}", clock.get_time());
        println!("-----------------------------------");
        for pos in positions.join() {
            println!("{:#?}", pos);
        }
        for (light) in (&lights).join() {
            println!("{:#?} {:#?}", light.color, light.time);
        }
    }
}
