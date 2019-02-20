use std::collections::HashMap;
use specs::prelude::*;
use crate::{IObservable, IObserver, Light, Index};

pub enum Event {
    TrafficLightColorChangeYellow,
    TrafficLightColorChangeRed
}

#[derive(Default)]
pub struct EventsManager {
    queue: HashMap<u64, Vec<u64>>
}

impl EventsManager {
    pub fn new() -> Self {
        Self {
            queue: HashMap::new()
        }
    }
    pub fn connect(&mut self, observable: u64, observer: u64) {
        self.queue.entry(observable).or_insert(Vec::new()).push(observer);
    }
    /*pub fn connect<T: >(&mut self, observable: &mut IObservable<Light>, observer: ) {
        *observer.subscribe(observable);
    }*/
}

pub struct EventsUpdate;
impl<'a> System<'a> for EventsUpdate {
    type SystemData = (
        Read<'a, EventsManager>,
        Entities<'a>,
        ReadStorage<'a, Index>,
        WriteStorage<'a, Light>
    );

    fn run(&mut self, (eventsmanager, entities, indexes, mut lights): Self::SystemData) {
        for (entity, index, light) in (&entities, &indexes, &mut lights).join() {
            println!("{:#?} {:#?} {:#?}", entity, index, light);
        }
    }
}