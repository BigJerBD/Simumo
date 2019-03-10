use specs::{Join, Read, ReadStorage, System};

use crate::components::dynamic::*;
use crate::components::statics::trafficlight::*;
use crate::ressources::clock;
use crate::systems::mobility::MobilitySystem;
use crate::systems::sys_prelude::*;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

#[simusystem]
pub struct PrintSystem;

impl SystemType for PrintSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        hook.add(self)
    }

    fn typename() -> String {
        String::from("PrintSystem")
    }

    fn system_name(&self) -> String {
        String::from("PrintSystem")
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![MobilitySystem::typename()]
    }
}

impl<'a> System<'a> for PrintSystem {
    type SystemData = (
        Read<'a, clock::Clock>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Light>,
    );

    fn run(&mut self, (clock, positions, lights): Self::SystemData) {
        println!("Simulation state at {:#?}", clock.get_time());
        println!("-----------------------------------");
        for pos in positions.join() {
            println!("{:#?}", pos);
        }
        for light in (&lights).join() {
            println!("{:#?} {:#?}", light.color, light.time);
        }
    }
}
