use crate::systems::clock::standard_clock::StandardClockSys;
use crate::systems::sys_prelude::*;

pub trait ClockSysType {}
impl SystemTypeDefinition for ClockSysType {
    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.clock = name;
    }

    fn get_dependencies(_dependencies: &SystemDependencies) -> Vec<String> {
        vec![]
    }

    fn build_subsystem(name: String) -> Result<Box<Any>, InvalidNameError> {
        match name.as_str() {
            StandardClockSys::typename => Ok(StandardClockSys.in_anybox()),
            _ => Err(invalid_system(&name))
        }
    }
}
