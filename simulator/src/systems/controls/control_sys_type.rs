use crate::systems::sys_prelude::*;

pub trait ControlSysType {}
impl SystemTypeDefinition for ControlSysType {
    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.controls.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        dependencies.agents.clone()
    }

    fn build_subsystem(name: String) -> Result<Box<Any>, InvalidNameError> {
        match name.as_str() {
            //todo :: add control systems
            _ => Err(invalid_system(&name))
        }
    }
}