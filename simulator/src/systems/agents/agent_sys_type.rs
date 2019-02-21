use crate::systems::sys_prelude::*;
use crate::systems::agents::AcceleratingAgentSys;


pub trait AgentSysType {

}

impl SystemTypeDefinition for AgentSysType {
    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.agents.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.clock.clone()]
    }

    fn build_subsystem(name: String) -> Result<Box<Any>, InvalidNameError> {
        match name.as_str() {
            AcceleratingAgentSys::typename => Ok(AcceleratingAgentSys.in_anybox()),
            _ => Err(invalid_system(&name))
        }
    }
}