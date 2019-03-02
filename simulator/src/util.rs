use dim::si::M;

use crate::components::dynamic::Position;
use crate::types::Geolocation;

pub fn geolocation_to_world_position(_geolocation: Geolocation) -> Position {
    Position {
        x: 0.0 * M,
        y: 0.0 * M,
    }
}
