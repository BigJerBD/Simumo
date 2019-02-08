use specs::prelude::{Component, VecStorage, World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use simumo_derive::simucomponent_data;
use crate::components::simumo_component::SimumoComponent;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Length(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Mass(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Identifier(pub i32);

//entity types
#[simucomponent_tag]
#[storage(VecStorage)]
pub struct CarType;

#[simucomponent_tag]
#[storage(VecStorage)]
pub struct BikeType;
