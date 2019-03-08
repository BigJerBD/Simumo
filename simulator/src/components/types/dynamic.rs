use crate::components::type_prelude::*;
use crate::metrics::Fdim;

use dim::si::{Meter, MeterPerSecond, MeterPerSecond2};
use dim::si::{M, MPS, MPS2};

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Position {
    #[simumo_metric]
    #[serde(deserialize_with = "meter_deserialize")]
    pub x: Meter<Fdim>,
    #[simumo_metric]
    #[serde(deserialize_with = "meter_deserialize")]
    pub y: Meter<Fdim>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            x: 0.0 * M,
            y: 0.0 * M,
        }
    }
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
    #[serde(deserialize_with = "meterpersecond_deserialize")]
    pub val: MeterPerSecond<Fdim>,
}

impl Default for Speed {
    fn default() -> Self {
        Self { val: 0.0 * MPS }
    }
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Acceleration {
    #[simumo_metric]
    #[serde(deserialize_with = "meterpersecond2_deserialize")]
    pub val: MeterPerSecond2<Fdim>,
}

impl Default for Acceleration {
    fn default() -> Self {
        Self { val: 0.0 * MPS2 }
    }
}
