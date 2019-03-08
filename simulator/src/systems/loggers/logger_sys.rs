use std::collections::HashMap;

use crate::components::log_record::LogRecord;
use crate::ressources;
use crate::systems::loggers::LoggerType;
use crate::systems::sys_prelude::*;

/// LoggerSys that can be specialised with a
/// specific Logger
///
/// example :: CsvLogging, PrintLogging, JsonLogging, etc.
#[derive(Deserialize)]
pub struct LoggerSystem<L: LoggerType> {
    directory: String,
    names: Vec<String>,
    writers: HashMap<String, L>,
}

//impl<L: LoggerType> SystemDefinition for LoggerSys<L> {
//    fn initialize(
//        &mut self, config: HashMap<String, FieldValue>,
//        general_config: HashMap<String, String>) -> Result<(), InvalidNameError>
//    {
//        self.log_directory = general_config
//            .get("log_directory")
//            .ok_or(invalid_field("log_directory"))?.clone();
//
//
//        let paths = match config.get("log_writers")
//            .ok_or(invalid_field("log_writers"))? {
//            FieldValue::ArrayVal(val) => val,
//            FieldValue::StringVal(_) => panic!("error")
//        };
//        self.log_writers =  paths
//            .iter()
//            .map(|s| match s {
//                FieldValue::StringVal(val) => val,
//                FieldValue::ArrayVal(_) => panic!("error"),
//            })
//            .map(|fname| {
//                let full_path = Path::new(&self.log_directory).join(&fname);
//                let full_path = full_path.to_str().unwrap();
//                (fname.clone(), L::open(full_path))
//            })
//            .collect();
//
//
//
//        Ok(())
//    }
//}

impl<'a, L: LoggerType> System<'a> for LoggerSystem<L> {
    type SystemData = (Read<'a, ressources::Clock>, WriteStorage<'a, LogRecord>);

    /// the run process select the right logger for every
    /// records
    fn run(&mut self, (_clock, mut records): Self::SystemData) {
        for record in records.join() {
            let logkey = record.get_type();
            match self.writers.get_mut(logkey) {
                Some(writer) => writer.write(record),
                None => panic!("Invalid log type {}", logkey),
            }
        }

        records.clear();
    }
}
