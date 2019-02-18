use dim::si::{S, Second};

use crate::metrics::Fdim;

pub struct Clock {
    pub dt: Second<Fdim>,
    tick: i32,
}

impl Clock {
    pub fn new(dt: Second<Fdim>) -> Clock {
        Clock { dt, tick: 0 }
    }
    pub fn update(&mut self) {
        self.tick += 1;
    }
    pub fn get_time(&self) -> Second<Fdim> {
        self.dt * f64::from(self.tick)
    }
}

impl Default for Clock {
    fn default() -> Self {
        Self{ dt: 1.0 * S, tick: 0 }
    }
}
