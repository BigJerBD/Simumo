use specs::prelude::{Component, VecStorage, World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use simumo_derive::simucomponent_data;
use super::super::simumo_component::SimumoComponent;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct EnergyControl(pub i32);
