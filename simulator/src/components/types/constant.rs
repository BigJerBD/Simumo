use crate::components::type_prelude::*;
use crate::metrics::Fdim;

use dim::si::{Kilogram, Meter};

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Length {
    #[simumo_metric]
    #[serde(deserialize_with = "meter_deserialize")]
    pub val: Meter<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Mass {
    #[simumo_metric]
    #[serde(deserialize_with = "kilogram_deserialize")]
    pub val: Kilogram<Fdim>,
}

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

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}
