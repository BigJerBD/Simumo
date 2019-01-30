use super::RoadId;
use crate::types::{Kilometer, KilometerPerHour, Meter};

#[derive(Clone, Debug)]
pub struct Road {
    pub id: RoadId,
    name: String,
    nb_lanes: u32,
    is_one_way: bool,
    lane_width: Meter,
    length: Kilometer,
    max_speed: KilometerPerHour,
    // A Bezier curve representation for the shape of the road?
    // Orientation?
}

impl Road {
    pub fn new() -> Self {
        Self {
            id: RoadId::end(),
            name: String::from(""),
            nb_lanes: 2,
            is_one_way: false,
            lane_width: Meter(5.0),
            length: Kilometer(1.0),
            max_speed: KilometerPerHour(50.0),
        }
    }
}
