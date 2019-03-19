use dim::si::M;

use crate::components::dynamic::Position;
use crate::types::Geolocation;

pub fn geolocation_to_world_position(_geolocation: Geolocation) -> Position {
    Position {
        x: 0.0 * M,
        y: 0.0 * M,
    }
}

pub fn measure(lat: f64, lon: f64) -> f64 {
    //let d2r: f64 = lon.to_radians();
    const PI: f64 = std::f64::consts::PI;
    const R: f64 = 6_378_137.;
    let lon: f64 = 0.0;
    let x: f64 = R * lat.to_radians().cos() * lon.to_radians().cos();
    let y: f64 = R * lat.to_radians().cos() * lon.to_radians().sin();
    println!("{} {}", lon, x);
    println!("{} {}", lat, y);
    1000.
}
