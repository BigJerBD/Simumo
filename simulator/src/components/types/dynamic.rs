use dim::si::{MeterPerSecond, MeterPerSecond2};
use dim::si::{MPS, MPS2};
use serde::ser::Serialize;
use serde::ser::Serializer;
use serde::ser::SerializeSeq;
use simumo_derive::{simucomponent_data, SimumoSerialize};
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

use crate::commons::LogDataEntry;
use crate::commons::metrics::Fdim;
use crate::commons::metrics::meterpersecond2_deserialize;
use crate::commons::metrics::meterpersecond_deserialize;

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
