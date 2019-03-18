use std::ops::Sub;

pub type Latitute = f32;
pub type Longitute = f32;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Geolocation(pub Latitute, pub Longitute);

impl Sub for Geolocation {
    type Output = Geolocation;

    fn sub(self, other: Geolocation) -> Self::Output {
        Geolocation(self.0 - other.0, self.1 - other.1)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Meter(pub f32);

#[derive(Clone, Copy, Debug)]
pub struct Kilometer(pub f32);

#[derive(Clone, Copy, Debug)]
pub struct KilometerPerHour(pub f32);
