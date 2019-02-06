use serde::{Deserialize, Serialize};
use specs::prelude::{Component, VecStorage};

use serde::Serializer;
use specs::DenseVecStorage;
use specs::Entity;
use std::fmt::Debug;

use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Component, Serialize)]
#[storage(VecStorage)]
pub struct LogRecord {
    //todo should be an array of different struct s
    // this way we could combine them
    log_data: Box<LogWritable>,
}

impl LogRecord {
    pub fn new(data: Box<LogWritable>) -> Self {
        LogRecord { log_data: data }
    }
    pub fn get_data(&self) -> &Box<LogWritable> {
        &self.log_data
    }
}

impl LogWritable for LogRecord {
    fn get_logtype(&self) -> String {
        (&self.log_data).get_logtype()
    }
}

// this erase_serde and serialize_trait_object allow to serialize
// trait object even though they dont have concrete data

pub trait LogWritable: Send + Sync + erased_serde::Serialize {
    fn get_logtype(&self) -> String;
}

impl<T: TypeInfo + Send + Sync + Serialize> LogWritable for T {
    fn get_logtype(&self) -> String {
        T::type_name()
    }
}
serialize_trait_object!(LogWritable);

// warning :: this is considered a bad solution
// log record should be able to combine multiple component
#[derive(TypeInfo, Serialize)]
pub struct CarPositionLog {
    pub id: u32,
    pub y: f32,
    pub x: f32,
}
