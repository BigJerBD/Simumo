use crate::components::constant::CarType;
use crate::components::constant::Identifier;
use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::components::statics::trafficlight::Light;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;

#[simusystem]
#[derive(Default)]
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Light>,
        ReadStorage<'a, Identifier>
    );

    fn run(&mut self, (clock, lights, identifiers): Self::SystemData) {
        for (light, id) in (&lights, &identifiers).join() {
            println!("{}: {:#?}, {:#?}", clock.get_time(), id, light);
        }
    }
}
