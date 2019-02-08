use crate::systems::logging::loggers::logger_impl::LoggerImpl;
use serde::Serialize;
use serde::Serializer;
use std::fs::File;
use std::result::Result;

/// Logger that writes data in a csv format in a specified file
pub struct CsvLogger {
    csv_write: csv::Writer<File>,
}

impl LoggerImpl for CsvLogger {
    fn open(filename: &str) -> Self {
        let filename = [filename, ".csv"].concat();

        let file = File::create(filename.to_string()).unwrap();
        CsvLogger {
            csv_write: csv::Writer::from_writer(file),
        }
    }

    fn write<S: Serialize>(&mut self, record: S) {
        self.csv_write.serialize(record);
    }
}
