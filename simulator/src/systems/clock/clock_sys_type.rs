use crate::systems::clock::standard_clock::StandardClockSystem;
use crate::systems::sys_prelude::*;

pub trait ClockSystemType {}
impl SystemTypeDefinition for ClockSystemType {
    type SubSystems = ClockSystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.clock = name;
    }

    fn get_dependencies(_dependencies: &SystemDependencies) -> Vec<String> {
        vec![]
    }
}

#[derive(Deserialize)]
#[serde(tag="type")]
pub enum ClockSystems {
    StandardClockSys(StandardClockSystem),
}
