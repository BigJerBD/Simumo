use crate::components::type_prelude::*;
use crate::metrics::Fdim;
use dim::si::Second;

#[simucomponent_base]
#[derive(Serialize)]
#[storage(VecStorage)]
pub struct LogRecord {
    #[serde(serialize_with = "timestamp_serialize")]
    timestamp: Second<Fdim>,
    record_id: u32,
    record_type: String,
    #[serde(flatten)]
    log_data: Box<LogWritable>,
}

impl LogRecord {
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
    pub fn get_type(&self) -> &String {
        &self.record_type
    }
}

fn timestamp_serialize<S>(x: &Second<Fdim>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_f64(x.value_unsafe)
}

pub trait LogWritable: Send + Sync + erased_serde::Serialize {}

impl<T: Send + Sync + Serialize> LogWritable for T {}
serialize_trait_object!(LogWritable);
