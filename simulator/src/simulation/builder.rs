use specs::DispatcherBuilder;
use specs::World;

pub struct SimulationBuilder<'a, 'b> {
    world: World,
    dispatcher: DispatcherBuilder<'a, 'b>,
}

impl<'a, 'b> SimulationBuilder<'a, 'b> {
    pub fn new() -> Self {
        Self {
            world: World::new(),
            dispatcher: DispatcherBuilder::new(),
        }
    }

    pub fn add_recorder() {}

    pub fn add_agent() {}
}
