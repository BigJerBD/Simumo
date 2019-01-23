pub struct Clock {
    _dt: f64,
    _tick: i32
}

impl Clock {
    pub fn new(dt: f64) -> Clock {
        Clock {
            _dt: dt,
            _tick: 0,
        }
    }
    pub fn update(&mut self) {
        self._tick += 1;
    }
    pub fn get_time(&self) -> f64 {
        self._dt * (self._tick as f64)
    }
}