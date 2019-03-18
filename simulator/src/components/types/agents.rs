use super::super::simumo_component::LogDataEntry;

use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use simumo_derive::{simucomponent_data, SimumoSerialize};
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;


#[simucomponent_data]
#[storage(VecStorage)]
pub struct AcceleratingAgent {
    pub is_decelerating: bool,
}
