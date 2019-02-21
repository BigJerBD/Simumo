use crate::systems::sys_prelude::*;

pub trait LoggerSysType {


}
impl SystemTypeDefinition for LoggerSysType {
    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.logger = name;
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        dependencies.controls.clone()
    }

    fn build_subsystem(name: String) -> Result<Box<Any>, InvalidNameError> {
        match name.as_str() {
            //todo
            _ => Err(invalid_system(&name))
        }
    }
}