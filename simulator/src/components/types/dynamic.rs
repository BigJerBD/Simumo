use crate::metrics::Fdim;
use crate::components::type_prelude::*;

use dim::si::{Meter, MeterPerSecond, MeterPerSecond2};

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Position {
    #[simumo_metric]
    #[serde(deserialize_with="meter_deserialize")]
    pub x: Meter<Fdim>,
    #[simumo_metric]
    #[serde(deserialize_with="meter_deserialize")]
    pub y: Meter<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Angle {
    pub val: Fdim,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Speed {
    #[simumo_metric]
    #[serde(deserialize_with="meterpersecond_deserialize")]
    pub val: MeterPerSecond<Fdim>,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Acceleration {
    #[simumo_metric]
    #[serde(deserialize_with="meterpersecond2_deserialize")]
    pub val: MeterPerSecond2<Fdim>,
}
