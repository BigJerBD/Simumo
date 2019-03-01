use crate::systems::agents::AgentSystems;
use crate::systems::clock::ClockSystems;
use crate::systems::controls::ControlSystems;
use crate::systems::loggers::LoggerSystems;
use crate::systems::mobility::MobilitySystems;
use crate::systems::physic::PhysicSystems;
use crate::systems::recorders::RecorderSystems;

#[derive(Deserialize)]
pub struct SystemsConfiguration {
    pub agents: Vec<AgentSystems>,
    pub clock: ClockSystems,
    pub controls: Vec<ControlSystems>,
    pub loggers: LoggerSystems,
    pub mobility: MobilitySystems,
    pub physics: PhysicSystems,
    pub recorders: Vec<RecorderSystems>,
}
