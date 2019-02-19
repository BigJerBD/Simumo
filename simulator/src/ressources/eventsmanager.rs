pub enum Event {
    TrafficLightColorChangeYellow,
    TrafficLightColorChangeRed
}

#[derive(Default)]
pub struct EventsManager {
    queue: Vec<Event>
}

impl EventsManager {
    pub fn new() -> Self {
        Self {
            queue: Vec::new()
        }
    }

    /*pub fn update(&mut self) {
        self._tick += 1;
    }
    pub fn get_dt(&self) -> f64 {
        self._dt
    }
    pub fn get_time(&self) -> f64 {
        self._dt * f64::from(self._tick)
    }*/
}