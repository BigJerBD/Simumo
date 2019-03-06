use crate::components::constant::Identifier;
use crate::ressources::clock;
use crate::systems::sys_prelude::*;

#[simusystem]
#[derive(Default)]
pub struct PrintSystem;

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        Entities<'a>,
        ReadStorage<'a, Identifier>
    );

    fn run(&mut self, (clock, entities, identifiers): Self::SystemData) {
        for (entity, id) in (&entities, &identifiers).join() {
            println!("{}: {:#?}", clock.get_time(), id);
        }
    }
}
