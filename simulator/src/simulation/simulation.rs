use specs::prelude::*;
use specs::Dispatcher;
use crate::ressources::clock;
use crate::ressources::generals;

pub struct Simulation<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
}


impl<'a, 'b> Simulation<'a, 'b> {
    pub fn run_simulation(&mut self) {
        loop {
            self.dispatcher.dispatch(&mut self.world.res);
            // Maintain dynamically add and remove entities in dispatch.
            self.world.maintain();

            if has_ended(&self.world) { break; }
        }
    }
}


fn has_ended(ressources: &World) -> bool {
    let clock = ressources.read_resource::<clock::Clock>();
    let end_time = ressources.read_resource::<generals::EndTime>();
    return clock.get_time() >= end_time.0;
}