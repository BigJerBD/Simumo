use crate::data_writer::DataWrite;

/// Configuration used to create
///
pub struct LoggerConfiguration {
    pub name: String,
    pub log_type: Box<DataWrite>,
}
