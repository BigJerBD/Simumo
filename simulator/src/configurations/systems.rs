use crate::systems::agents::AgentSystems;

#[derive(Deserialize, Default)]
pub struct SystemsConfiguration {
    pub agents: Vec<AgentSystems>,
}
