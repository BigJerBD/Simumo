use crate::types::{Geolocation, Latitute, Longitute};

#[derive(Clone, Debug)]
pub struct Spline;

impl Spline {
    pub fn new(_key_points: &[Geolocation]) -> Self {
        Self {}
    }

    pub fn get_location_at_distance_along_spline(&self, _distance: f64) -> Geolocation {
        Geolocation(Longitute(0.0), Latitute(0.0))
    }
}
