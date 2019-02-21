use crate::systems::recorders::CarPosRec;
use crate::systems::sys_prelude::*;
use crate::systems::system_type_definition::SystemTypeDefinition;

pub trait RecorderSysType {}

impl SystemTypeDefinition for RecorderSysType {
    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.recorders.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.mobility.clone()]
    }

    fn build_subsystem(name: String) -> Result<Box<Any>, InvalidNameError> {
        match name.as_str() {
            CarPosRec::typename => Ok(CarPosRec::default().in_anybox()),
            _ => Err(invalid_system(&name))
        }
    }
}


