use dim::si::{Meter, MeterPerSecond, MeterPerSecond2};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use simumo_derive::*;
use specs::prelude::{Component, VecStorage, World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;

use crate::components::simumo_component::*;
use crate::metrics::Fdim;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Position {
    #[simumo_metric]
    pub x: Meter<Fdim>,
    #[simumo_metric]
    pub y: Meter<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Angle{
    pub val: Fdim,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Speed{
    #[simumo_metric]
    pub val: MeterPerSecond<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Acceleration{
    #[simumo_metric]
    pub val: MeterPerSecond2<Fdim>,
}
