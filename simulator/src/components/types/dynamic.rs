use specs::prelude::{Component, VecStorage,World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use simumo_derive::simucomponent_base;
use crate::components::simumo_component::SimumoComponent;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Angle(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Speed(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Acceleration(pub f32);
