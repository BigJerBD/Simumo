pub struct Clock {
    dt: f64,
    tick: i32
}
impl Clock {
    pub fn new(dt: f64) -> Clock {
        Clock {
            dt,
            tick: 0,
        }
    }
    pub fn update(&mut self) {
        self.tick += 1;
    }
    pub fn get_time(&self) -> f64 {
        self.dt * (self.tick as f64)
    }
}