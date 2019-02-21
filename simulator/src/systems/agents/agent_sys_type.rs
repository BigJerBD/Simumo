use crate::systems::sys_prelude::*;
use crate::systems::agents::AcceleratingAgentSystem;


pub trait AgentSystemType {}
impl SystemTypeDefinition for AgentSystemType {
    type SubSystems = AcceleratingAgentSystem;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.agents.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.clock.clone()]
    }
}


enum AgentSystems{
    AcceleratingAgentSys(AcceleratingAgentSystem)
}