use std::collections::HashMap;
use specs::prelude::*;
use crate::{Identifier, EntityTable, IObservable, IObserver};
use crate::{Light, TrafficLightColor};
extern crate shrev;
use shrev::EventChannel;

mod observables;
pub use observables::{Observers};

#[derive(Clone)]
pub enum Event {
    TrafficLightColorChange(TrafficLightColor)
}

#[derive(Default)]
pub struct EventsManager {
    hooks: HashMap<String, Vec<String>>,
    events: HashMap<String, EventChannel<&'static Event>>,
    reader_ids: HashMap<String, ReaderId<&'static Event>>
}

impl EventsManager {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            events: HashMap::new(),
            reader_ids: HashMap::new()
        }
    }
    pub fn connect(&mut self, observable: String, observer: String) {
        self.hooks.entry(observable).or_insert(Vec::new()).push(observer);
    }
    pub fn get_observables(&self, id: &str) -> Vec<String> {
        match self.hooks.get(id) {
            Some(observables) => observables.clone(),
            None => Vec::new()
        }
    }
    pub fn add_event(&mut self, idObservable: &str, event: &'static Event) {
        let mut channel = EventChannel::new();
        let mut reader_id = channel.register_reader();
        self.events.entry(idObservable.to_string()).or_insert(channel).single_write(event);
        self.reader_ids.entry(idObservable.to_string()).or_insert(reader_id);
    }
    pub fn get_events(&mut self, idObservable: String) -> Vec<&Event> {
        let mut readerId: Option<&ReaderId<&Event>> = self.reader_ids.get(&idObservable);
        match readerId {
            Some(mut rId) => {
                let mut channel = self.events.entry(idObservable.to_string()).or_default();
                channel.read(&mut rId).cloned().collect::<Vec<_>>()
            },
            None => Vec::new()
        }
    }
}

pub struct EventsHookUpdate;
impl<'a> System<'a> for EventsHookUpdate {
    type SystemData = (
        Read<'a, EventsManager>,
        Read<'a, EntityTable>,
        Entities<'a>,
        ReadStorage<'a, Identifier>,
        WriteStorage<'a, Observers>
    );

    fn run(&mut self, (eventsmanager, entitytable, entities, identifiers, mut observables): Self::SystemData) {
        /*for (entity, identifier, observableslist) in (&entities, &identifiers, &mut observables).join() {
            let currentObservables: &Vec<&Entity> = observableslist.get_list();
            for currentObservable in currentObservables {
                
            }
        }*/
    }
}

pub struct EventsUpdate;
impl<'a> System<'a> for EventsUpdate {
    type SystemData = (
        Write<'a, EventsManager>,
        Entities<'a>,
        ReadStorage<'a, Identifier>,
        ReadStorage<'a, Observers>,
        WriteStorage<'a, Light>
    );

    fn run(&mut self, (mut eventsmanager, entities, identifiers, observers, mut lights): Self::SystemData) {
        for (entity, identifier, observerslist, light) in (&entities, &identifiers, &observers, &mut lights).join() {
            let events: Vec<&Event> = eventsmanager.get_events(identifier.0.to_string());
            let currentObservers: &Vec<&Entity> = observerslist.get_list();
            for currentObserver in currentObservers {
                for event in events.iter() {
                    light.notify(event);
                }
            }
        }
    }
}