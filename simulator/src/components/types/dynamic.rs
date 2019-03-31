/*! Define a dynamic component. */

use crate::components::simumo_component::LogDataEntry;
use crate::metrics::Fdim;
use crate::metrics::{meter_deserialize, meterpersecond2_deserialize, meterpersecond_deserialize};

use dim::si::{Meter, MeterPerSecond, MeterPerSecond2};
use dim::si::{M, MPS, MPS2};
use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use simumo_derive::{simucomponent_data, SimumoSerialize};
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

/// Position on the map of the component.
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


///Direction of the component. 0=East, 90=North, 180=West, 270=South
#[simucomponent_data]
#[storage(VecStorage)]
pub struct Angle {
    pub val: Fdim,
}

impl Default for Angle {
    fn default() -> Self {
        Self { val: 0.0}
    }
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
