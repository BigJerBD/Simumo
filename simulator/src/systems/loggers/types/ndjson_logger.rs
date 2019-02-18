use std::fs::File;
use std::io::Write;

use serde::Serialize;

use crate::systems::loggers::LoggerType;
use crate::systems::sys_prelude::*;

/// Logger that writes data in a json format in a specified file
#[simusystem]
pub struct NdJsonLogger {
    file_writer: File,
}

impl LoggerType for NdJsonLogger {
    fn open(filename: &str) -> Self {
        let filename = [filename, ".ndjson"].concat();
        Self {
            file_writer: File::create(filename.to_string()).unwrap(),
        }
    }

    fn write<S: Serialize>(&mut self, record: S) {
        let mut json = serde_json::to_string(&record).unwrap();
        json.push('\n');
        self.file_writer.write(json.as_bytes()).unwrap();
    }
}
