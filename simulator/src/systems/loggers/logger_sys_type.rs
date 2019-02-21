use crate::systems::loggers::types::CsvLogger;
use crate::systems::loggers::types::NdJsonLogger;
use crate::systems::loggers::LoggerSystem;
use crate::systems::sys_prelude::*;

pub trait LoggerSystemType {}
impl SystemTypeDefinition for LoggerSystemType {
    type SubSystems = LoggerSystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.logger = name;
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        dependencies.controls.clone()
    }
}

#[derive(Deserialize)]
pub enum LoggerSystems {
    CsvLogger(LoggerSystem<CsvLogger>),
    NdJsonLogger(LoggerSystem<NdJsonLogger>),
    PrintLogger(LoggerSystem<NdJsonLogger>),
}
