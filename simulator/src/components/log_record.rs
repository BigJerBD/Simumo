/*! Define a log record . */

use crate::commons::metrics::Fdim;

use dim::si::Second;
use serde::ser::Serialize;
use serde::ser::Serializer;

#[derive(Serialize)]
pub struct LogRecord {
    #[serde(serialize_with = "timestamp_serialize")]
    timestamp: Second<Fdim>,
    record_id: u32,
    record_type: String,
    log_data: Box<LogWritable>,
}

impl LogRecord {
    ///Create a new log record containing the given value.
    pub fn new(
        timestamp: Second<Fdim>,
        record_id: u32,
        record_type: String,
        log_data: Box<LogWritable>,
    ) -> Self {
        LogRecord {
            timestamp,
            record_id,
            record_type,
            log_data,
        }
    }
    ///Return current type.
    pub fn get_type(&self) -> &String {
        &self.record_type
    }
}

///Makes the timestamp in the log record serializable.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn timestamp_serialize<S>(x: &Second<Fdim>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    //todo make timestamp into a 00:00:00 format
    s.serialize_f64(x.value_unsafe)
}

pub trait LogWritable: Send + Sync + erased_serde::Serialize {}

impl<T: Send + Sync + Serialize> LogWritable for T {}
serialize_trait_object!(LogWritable);
