use crate::metrics::Fdim;

use dim::si::Second;
use serde::ser::Serialize;
use serde::ser::Serializer;
use simumo_derive::simucomponent_base;
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;


#[simucomponent_base]
#[derive(Serialize)]
#[storage(VecStorage)]
pub struct LogRecord {
    #[serde(serialize_with = "timestamp_serialize")]
    timestamp: Second<Fdim>,
    record_id: u32,
    record_type: String,
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
