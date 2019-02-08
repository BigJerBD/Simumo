use serde::{Deserialize, Serialize};
use specs::prelude::{Component, VecStorage};

use serde::ser::SerializeStruct;
use serde::Serializer;
use specs::DenseVecStorage;
use specs::Entity;
use std::fmt::Debug;

use crate::components::dynamic::Position;
use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Component, Serialize)]
#[storage(VecStorage)]
pub struct LogRecord {
    timestamp: f64,
    record_id: u32,
    record_type: String,
    #[serde(flatten)]
    log_data: Box<LogWritable>,
}

impl LogRecord {
    pub fn new(
        timestamp: f64,
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

pub trait LogWritable: Send + Sync + erased_serde::Serialize {}
impl<T: Send + Sync + Serialize> LogWritable for T {}
serialize_trait_object!(LogWritable);
