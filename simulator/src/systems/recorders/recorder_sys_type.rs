use crate::systems::recorders::CarPosRecSystem;
use crate::systems::sys_prelude::*;
use crate::systems::system_type_definition::SystemTypeDefinition;

pub trait RecorderSystemType {}

impl SystemTypeDefinition for RecorderSystemType {
    type SubSystems = RecorderSystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.recorders.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.mobility.clone()]
    }
}

#[derive(Deserialize)]
#[serde(tag="type")]
pub enum RecorderSystems {
    CarPosRecSystem(CarPosRecSystem),
}
