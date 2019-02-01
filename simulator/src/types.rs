#[derive(Clone, Debug)]
pub struct Meter(pub f64);

#[derive(Clone, Debug)]
pub struct Kilometer(pub f64);

#[derive(Clone, Debug)]
pub struct KilometerPerHour(pub f64);

pub struct Longitute(pub f64);

pub struct Latitute(pub f64);

pub struct Geolocation(pub Longitute, pub Latitute);
