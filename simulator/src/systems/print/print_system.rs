use crate::components::constant::Identifier;
use crate::components::dynamic::Position;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;

#[simusystem]
#[derive(Default)]
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Identifier>,
    );

    fn run(&mut self, (clock, positions, identifiers): Self::SystemData) {
        for (position, id) in (&positions, &identifiers).join() {
            println!("{}: {:#?}, {:#?}", clock.get_time(), id, position);
        }
    }
}
