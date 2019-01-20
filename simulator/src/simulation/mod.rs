mod clock;
use self::clock::*;
mod eventsmanager;
use self::eventsmanager::*;

pub struct Simulation {
    _clock: Clock,
    _eventsmanager: EventsManager,
    _maxtime: f64
}

impl Simulation {
    pub fn new(dt: f64, maxtime: f64) -> Simulation {
        Simulation {
            _clock: Clock::new(dt),
            _eventsmanager: EventsManager::new(),
            _maxtime: maxtime
        }
    }
    pub fn update_clock(&mut self) {
        if !self.has_ended() {
            self._clock.update();
        }
    }
    pub fn execute_events(&self) {
        self._eventsmanager.execute(self.get_time());
    }
    pub fn update_objects(&self) {
        println!("Updating simulation objects...");
    }
    pub fn get_state(&self) {
        println!("This function should return a 'SimulationState' type object representing the current state of the simulation");
    }
    pub fn has_ended(&self) -> bool {
        self.get_time() >= self._maxtime
    }
    pub fn get_time(&self) -> f64 {
        self._clock.get_time()
    }
}