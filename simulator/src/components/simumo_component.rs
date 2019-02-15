use specs::Component;
use specs::World;
use std::collections::HashMap;
use typeinfo::TypeInfo;

pub trait SimumoComponent: TypeInfo + Component {
    fn register(&self, simulation: &mut World, isdone: &mut bool);
}

pub trait SimumoLoggable {
    fn to_log(&self) -> HashMap<String, f32>;
}

pub trait SimumoSerialize {}
