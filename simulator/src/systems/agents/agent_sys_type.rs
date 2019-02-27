use crate::systems::agents::AcceleratingAgentSystem;
use crate::systems::sys_prelude::*;

pub trait AgentSystemType {}
impl SystemTypeDefinition for AgentSystemType {
    type SubSystems = AgentSystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.agents.push(name);
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.clock.clone()]
    }
}

#[derive(Deserialize)]
#[serde(tag="type")]
pub enum AgentSystems {
    Accelerating(AcceleratingAgentSystem),
}
