use crate::types::{Geolocation, Latitute, Longitute};

#[derive(Clone, Debug)]
pub struct Spline;

impl Spline {
    pub fn new(key_points: &[Geolocation]) -> Self {
        Self {}
    }

    pub fn get_location_at_distance_along_spline(&self, distance: f64) -> Geolocation {
        Geolocation(Longitute(0.0), Latitute(0.0))
    }
}
