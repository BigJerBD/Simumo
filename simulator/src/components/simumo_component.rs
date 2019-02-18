use std::collections::HashMap;

pub trait SimumoLoggable {
    fn to_log(&self) -> HashMap<String, f32>;
}

pub trait SimumoSerialize {}
