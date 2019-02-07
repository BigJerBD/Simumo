use crate::components::dynamic::*;
use crate::components::log_record::{LogRecord, LogWritable};
use crate::ressources::{clock, generals};

use specs::prelude::*;

use std::collections::HashMap;
use std::fs::File;

use crate::systems::logging::loggers::logger_impl::LoggerImpl;
use csv::Writer;
use std::path::Path;

/// LoggerSys that can be specialised with a
/// specific Logger
///
/// example :: CsvLogging, PrintLogging, JsonLogging, etc.
///
pub struct LoggerSys<L: LoggerImpl> {
    dir_path: String,
    log_writers: HashMap<String, L>,
}

impl<L: LoggerImpl> LoggerSys<L> {
    /// Init by opening a logger for every log
    /// data type input for the function
    pub fn new(dir_path: String, logtypes: &[&str]) -> LoggerSys<L> {
        let log_writers: HashMap<_, _> = logtypes
            .iter()
            .map(|s| s.to_string())
            .map(|fname| {
                let full_path = Path::new(&dir_path).join(&fname);
                let full_path = full_path.to_str().unwrap();
                (fname.clone(), L::open(full_path))
            })
            .collect();

        LoggerSys {
            dir_path,
            log_writers,
        }
    }
}

impl<'a, L: LoggerImpl> System<'a> for LoggerSys<L> {
    type SystemData = (Read<'a, clock::Clock>, WriteStorage<'a, LogRecord>);

    /// the run process select the right logger for every
    /// records
    fn run(&mut self, (clock, mut records): Self::SystemData) {
        for record in records.join() {
            let logkey = record.get_type();
            match self.log_writers.get_mut(&logkey) {
                Some(writer) => writer.write(record),
                None => panic!("Invalid log type {}", logkey),
            }
        }

        records.clear();
    }
}
