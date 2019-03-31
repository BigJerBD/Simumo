use std::fs::File;
use std::io::Write;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

/// used to create
/// specific Logger Implementation
pub trait LoggerType: Send + Sync {
    fn open(filename: &str) -> Self;
    fn write<S: Serialize>(&mut self, record: S);
}

/// Logger that writes data in a csv format in a specified file
///
///
pub struct CsvLogger {
    csv_write: csv::Writer<File>,
}
impl LoggerType for CsvLogger {
    fn open(filename: &str) -> Self {
        let filename = [filename, ".csv"].concat();

        let file = File::create(filename.to_string()).unwrap();
        CsvLogger {
            csv_write: csv::Writer::from_writer(file),
        }
    }

    fn write<S: Serialize>(&mut self, record: S) {
        self.csv_write.serialize(record).unwrap();
    }
}

/// Logger that writes data in a json format in a specified file
///
///
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
        self.file_writer.write_all(json.as_bytes()).unwrap();
    }
}
