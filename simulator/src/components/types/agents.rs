use crate::commons::CartesianCoord;
use crate::commons::LogDataEntry;
use serde::ser::Serialize;
use serde::ser::SerializeSeq;
use serde::ser::Serializer;
use simumo_derive::{simucomponent_base, simucomponent_data, SimumoSerialize};
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct AcceleratingAgent {
    pub is_decelerating: bool,
}

/*#[derive(Serialize, Deserialize, Debug)]
pub struct ImmobileAgent {
    pub id: Identifier,
    pub position: Position,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConstantSpeedAgent {
    pub id: Identifier,
    pub position: Position,
    pub speed: Speed,
}*/

#[simucomponent_base]
#[derive(Debug)]
#[storage(VecStorage)]
pub struct Destination {
    pub val: CartesianCoord,
}

impl Default for Destination {
    fn default() -> Self {
        Self {
            val: CartesianCoord::default(),
        }
    }
}
