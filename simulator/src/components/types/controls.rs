use crate::components::simumo_component::*;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use simumo_derive::*;
use specs::prelude::{Component, VecStorage, World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct EnergyControl(pub i32);
