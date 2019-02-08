use specs::prelude::{Component, VecStorage,World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use simumo_derive::simucomponent_base;
use crate::components::simumo_component::SimumoComponent;


use serde::{Deserialize, Serialize};
use serde::ser::SerializeStruct;
use serde::Serializer;
use specs::DenseVecStorage;
use specs::Entity;
use std::fmt::Debug;




#[simucomponent_base]
#[derive(Serialize)]
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
