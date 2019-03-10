use crate::systems::mobility::MobilitySystem;
use crate::systems::recorders::CarPositionRecorderSystem;
use crate::systems::sys_prelude::*;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum RecorderSystem {
    CarPositionRecorder(CarPositionRecorderSystem),
}
impl SystemType for RecorderSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            RecorderSystem::CarPositionRecorder(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("RecorderSystem")
    }
    fn system_name(&self) -> String {
        match self {
            RecorderSystem::CarPositionRecorder(s) => String::from(s.type_of()),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![MobilitySystem::typename()]
    }
}
