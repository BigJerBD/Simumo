use std::fs::File;

use serde::Deserialize;
use serde::Deserializer;
use serde::Serialize;

use crate::systems::loggers::LoggerType;

/// Logger that writes data in a csv format in a specified file
#[derive(Deserialize)]
pub struct CsvLogger {
    #[serde(deserialize_with = "deser_open")]
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

/// Function to open file instead of serializing it
fn deser_open<'de, D>(deserializer: D) -> Result<csv::Writer<File>, D::Error>
where
    D: Deserializer<'de>,
{
    let file = File::create(String::deserialize(deserializer)?).unwrap();
    Ok(csv::Writer::from_writer(file))
}
