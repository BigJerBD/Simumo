use serde::Deserialize;
use serde::Deserializer;
use serde::ser::Serialize;
use serde::ser::Serializer;
use serde::ser::SerializeStruct;
use simumo_derive::simucomponent_base;
use specs::prelude::{Component, VecStorage};
use typeinfo::TypeInfo;
use typeinfo_derive::TypeInfo;

use crate::commons::CartesianCoord;
use crate::commons::PolarCoord;

#[simucomponent_base]
#[derive(Debug)]
#[derive(Clone)]
#[storage(VecStorage)]
pub struct Position {
    pub val: CartesianCoord
}

impl Default for Position {
    fn default() -> Self {
        Self {
            val: CartesianCoord::from_float(
                0.0,
                0.0,
            )
        }
    }
}

//todo this might be useless and removed
impl<'de> Deserialize<'de> for Position {
    fn deserialize<D>(deserializer: D) -> Result<Position, D::Error>

        where D: Deserializer<'de> {
        let pos = PositionDeserialzable::deserialize(deserializer)?;
        Ok(
            Position {
                val: CartesianCoord::from_float(
                    pos.x, pos.y,
                )
            }
        )
    }
}

impl Serialize for Position {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where
        S: Serializer {
        // todo this function should be somewhere else
        // its a transformation of a record, it could be something configurable
        let pcoord= PolarCoord::from_cartesian(&self.val);
        let mut strct = serializer.serialize_struct("position", 2)?;
        strct.serialize_field("x", &pcoord.1)?;
        strct.serialize_field("y", &pcoord.0)?;
        strct.end()
    }
}


#[derive(Deserialize)]
struct PositionDeserialzable {
    pub x: f64,
    pub y: f64,
}
