use crate::systems::loggers::types::CsvLogger;
use crate::systems::loggers::types::NdJsonLogger;
use crate::systems::loggers::AbstractLoggerSystem;
use crate::systems::recorders::RecorderSystem;
use crate::systems::system_type::DispatcherBuilderHook;
use crate::systems::system_type::SystemType;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum LoggerSystem {
    CsvLogger(AbstractLoggerSystem<CsvLogger>),
    NdJsonLogger(AbstractLoggerSystem<NdJsonLogger>),
}
impl SystemType for LoggerSystem {
    fn setup(self, hook: &mut DispatcherBuilderHook) {
        match self {
            LoggerSystem::CsvLogger(s) => hook.add(s),
            LoggerSystem::NdJsonLogger(s) => hook.add(s),
        }
    }

    fn typename() -> String {
        String::from("LoggerSystem")
    }

    fn system_name(&self) -> String {
        match self {
            LoggerSystem::CsvLogger(_) => String::from("CsvLogger"),
            LoggerSystem::NdJsonLogger(_) => String::from("NdJsonLogger"),
        }
    }

    fn type_dependencies(&self) -> Vec<String> {
        vec![RecorderSystem::typename()]
    }
}
