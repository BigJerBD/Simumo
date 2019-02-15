use crate::components::simumo_component::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use simumo_derive::*;
use specs::prelude::{Component, VecStorage, World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;

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
