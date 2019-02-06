use serde::Serialize;
use serde::Serializer;
use std::fs::File;
use std::result::Result;

/// used to create
/// specific Logger Implementation
pub trait LoggerImpl {
    fn open(filename: &str) -> Self;
    fn write<S: Serialize>(&mut self, record: S);
}

/// Logger that writes data in a csv form in a specified file
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

/// Logger that simply print the serialized input
pub struct PrintLogger;

impl LoggerImpl for PrintLogger {
    fn open(filename: &str) -> Self {
        PrintLogger
    }

    fn write<S: Serialize>(&mut self, record: S) {}
}
