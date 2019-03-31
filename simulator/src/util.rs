use dim::si::M;

use crate::components::dynamic::Position;
use crate::types::Geolocation;

pub fn geolocation_to_world_position(_geolocation: Geolocation) -> Position {
    Position {
        x: 0.0 * M,
        y: 0.0 * M,
    }
}

pub fn polar_coordinates_to_cartesian(coords: (f64, f64)) -> (f64, f64) {
    let (lon, lat) = coords;
    const R: f64 = 6_378_137.;
    let x: f64 = R * lon.to_radians();// * lat.to_radians().cos();
    let y: f64 = R * lat.to_radians();
    (x, y)
}
