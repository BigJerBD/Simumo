use std::collections::HashMap;
use specs::prelude::*;
use crate::{Identifier};
use crate::{TrafficLightColor};

#[derive(Clone, Debug)]
pub enum Event {
    TrafficLightColorChange(TrafficLightColor)
}

#[derive(Default)]
pub struct EventsManager {
    hooks: HashMap<String, Vec<String>>,
    events_old: HashMap<String, Vec<&'static Event>>,
    events_new: HashMap<String, Vec<&'static Event>>
}

impl EventsManager {
    pub fn new() -> Self {
        Self {
            hooks: HashMap::new(),
            events_old: HashMap::new(),
            events_new: HashMap::new()
        }
    }
    pub fn connect(&mut self, id_observable: String, id_observer: String) {
        self.hooks.entry(id_observable).or_insert(Vec::new()).push(id_observer);
    }
    pub fn get_observers(&self, id_observable: &str) -> Vec<String> {
        match self.hooks.get(id_observable) {
            Some(observers) => observers.clone(),
            None => Vec::new()
        }
    }
    pub fn add_event_to_be_executed(&mut self, id_observable: &str, event: &'static Event) {
        self.events_new.entry(id_observable.to_string()).or_insert(Vec::new()).push(event);
    }
    pub fn get_events_to_execute(&self, id_observer: &str) -> Vec<&Event> {
        match self.events_old.get(id_observer) {
            Some(events) => events.clone(),
            None => Vec::new()
        }
    }
    pub fn swap_events(&mut self) {
        let mut events_reorganized_by_observer: HashMap<String, Vec<&Event>> = HashMap::new();
        for (id_observable, events) in self.events_new.iter() {
            let id_observers: Vec<String> = self.get_observers(id_observable.as_str());
            for id_observer in id_observers {
                for event in events.iter() {
                    events_reorganized_by_observer.entry(id_observer.to_string()).or_insert(Vec::new()).push(event);
                }
            }
        }
        self.events_old = events_reorganized_by_observer;
        self.events_new = HashMap::new();
    }
}

pub struct EventsHookUpdate;
impl<'a> System<'a> for EventsHookUpdate {
    type SystemData = (
        Write<'a, EventsManager>,
        Entities<'a>,
        ReadStorage<'a, Identifier>
    );

    fn run(&mut self, (mut eventsmanager, entities, identifiers): Self::SystemData) {
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
        Write<'a, EventsManager>
    );

    fn run(&mut self, mut eventsmanager: Self::SystemData) {
        eventsmanager.swap_events();
    }
}