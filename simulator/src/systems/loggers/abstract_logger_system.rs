use std::collections::HashMap;
use std::path::Path;

use crate::components::log_record::LogRecord;
use crate::ressources::clock;
use crate::systems::loggers::LoggerType;

use serde::Deserialize;
use serde::Deserializer;
use specs::prelude::{Read, System, WriteStorage, Join};

/// LoggerSys that can be specialised with a
/// specific Logger
///
/// example :: CsvLogging, PrintLogging, JsonLogging, etc.
#[derive(Default)]
pub struct AbstractLoggerSystem<L: LoggerType> {
    writers: HashMap<String, L>,
}

impl<L: LoggerType> AbstractLoggerSystem<L> {
    pub fn from_directory_struct(dir_struct: &DirectoryStructure) -> Self {
        Self {
            writers: dir_struct
                .files
                .iter()
                .zip(dir_struct.filepaths().iter())
                .map(|(fname, path)| (fname.clone(), L::open(path)))
                .collect(),
        }
    }
}

impl<'a, L: LoggerType> System<'a> for AbstractLoggerSystem<L> {
    type SystemData = (Read<'a, clock::Clock>, WriteStorage<'a, LogRecord>);

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

/// Deserialize Implementation based on a log structure
///
impl<'de, L: LoggerType> Deserialize<'de> for AbstractLoggerSystem<L> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let dir_struct = DirectoryStructure::deserialize(deserializer)?;
        Ok(AbstractLoggerSystem::from_directory_struct(&dir_struct))
    }
}

/// Directory collection
/// of files used for deserialization
#[derive(Deserialize)]
pub struct DirectoryStructure {
    pub directory: String,
    pub files: Vec<String>,
}

impl DirectoryStructure {
    /// give all the log filepaths for the logger
    ///
    fn filepaths(&self) -> Vec<String> {
        self.files
            .iter()
            .map(|file| Path::new(&self.directory).join(&file))
            .map(|path| String::from(path.to_str().unwrap()))
            .collect()
    }
}
//
