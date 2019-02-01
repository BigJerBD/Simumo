use crate::components::dynamic::Position;
use crate::types::Geolocation;

pub fn geolocation_to_world_position(geolocation: Geolocation) -> Position {
    Position { x: 0.0, y: 0.0 }
}
