use crate::systems::sys_prelude::*;

pub trait ControlSystemType {}

impl SystemTypeDefinition for ControlSystemType {
    type SubSystems = ControlSystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.controls.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        dependencies.agents.clone()
    }
}

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum ControlSystems {
    //
}
