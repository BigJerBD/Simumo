pub struct EventsManager {}

impl EventsManager {
    pub fn new() -> EventsManager {
        EventsManager {}
    }
    pub fn execute(&self, time: f64) {
        println!("Executing events at time {}s...", time);
    }
}
