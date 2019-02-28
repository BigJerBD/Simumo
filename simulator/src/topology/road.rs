use super::curve::Curve;
use super::RoadId;
use crate::types::{KilometerPerHour, Meter};

#[derive(Clone, Debug)]
pub struct Road {
    pub id: RoadId,
    pub name: String,
    pub lanes: Vec<Lane>,
}

impl Road {
    pub fn new() -> Self {
        Self {
            id: RoadId::end(),
            name: String::from(""),
            lanes: vec![],
        }
    }
}

#[derive(Clone, Debug)]
pub enum Direction {
    Inbound,
    OutBound,
}

#[derive(Clone, Debug)]
pub struct Lane {
    direction: Direction,
    width: Meter,
    max_speed: KilometerPerHour,
    curve: Curve,
}

impl Lane {}
