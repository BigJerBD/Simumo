use dim::si::{Kilogram, Meter};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use simumo_derive::*;
use specs::prelude::{Component, VecStorage, World};
use typeinfo::TypeInfo;
use typeinfo_derive::*;

use crate::components::simumo_component::*;
use crate::metrics::Fdim;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Length {
    #[simumo_metric]
    pub val: Meter<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Mass {
    #[simumo_metric]
    pub val: Kilogram<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Identifier(pub String);

//entity types
#[simucomponent_tag]
#[storage(VecStorage)]
pub struct CarType;

#[simucomponent_tag]
#[storage(VecStorage)]
pub struct BikeType;
