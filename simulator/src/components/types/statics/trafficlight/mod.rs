use crate::ressources::{clock};
use crate::eventsmanager::{Event, EventsUpdate, EventsManager};
use crate::{Identifier};

use dim::si::{S, Second};
use serde::ser::{Serialize, Serializer};
use simumo_derive::*;
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use crate::components::simumo_component::*;
use crate::metrics::Fdim;

extern crate hibitset;
extern crate shrev;
extern crate specs;
use specs::prelude::*;

#[derive(Copy, Clone, Debug, Serialize, PartialEq)]
pub enum TrafficLightColor { RED, YELLOW, GREEN, ORANGE }

#[derive(Component, TypeInfo, Debug)]
#[storage(VecStorage)]
pub struct Light {
    pub color: TrafficLightColor,
    pub max_green_time: Second<Fdim>,
    pub max_yellow_time: Second<Fdim>,
    pub time: Second<Fdim>,
    pub observers: Vec<LightObserver>
}

impl Light {
    pub fn new(color: TrafficLightColor, max_green_time: Second<Fdim>, max_yellow_time: Second<Fdim>, time: Second<Fdim>) -> Self {
        Self {
            color,
            max_green_time,
            max_yellow_time,
            time,
            observers: Vec::new()
        }
    }
    fn reset_to_green(&mut self) {
        self.color = TrafficLightColor::GREEN;
        self.time = self.max_green_time;
    }
    fn reset_to_yellow(&mut self) {
        self.color = TrafficLightColor::YELLOW;
        self.time = self.max_yellow_time;
    }
    fn reset_to_red(&mut self) {
        self.color = TrafficLightColor::RED;
        self.time = 0.0 * S;
    }
}

#[derive(Debug)]
pub enum LightObserver {
    Light(Light)
}

pub trait IObservable<T> {
    fn add_observer(&mut self, observer: &T);
    fn notify(&self, event: &Event);
}
pub trait IObserver<T> {
    fn subscribe(&self, observable: &mut T);
    fn update(&mut self, observable: &T, event: &Event);
}

impl IObservable<Light> for Light {
    fn add_observer(&mut self, observer: &Light) {
        //self.observers.push(LightObserver::Light(*observer));
    }
    fn notify(&self, event: &Event) {
        /*for observer in self.observers.iter() {
            match observer {
                LightObserver::Light(mut light) => light.update(self, event)
            }
        }*/
    }
}

impl IObserver<Light> for Light {
    fn subscribe(&self, observable: &mut Light) {
        observable.add_observer(self);
    }
    fn update(&mut self, observable: &Light, event: &Event) {
        match event {
            Event::TrafficLightColorChange(color) => {
                if color == &TrafficLightColor::RED {
                    self.reset_to_green();
                }
            }
        }
    }
}

pub struct LightUpdate;
impl<'a> System<'a> for LightUpdate {
    type SystemData = (
        Write<'a, EventsManager>,
        Entities<'a>,
        ReadStorage<'a, Identifier>,
        WriteStorage<'a, Light>,
        Read<'a, clock::Clock>
    );

    fn run(&mut self, (mut eventsmanager, entities, identifiers, mut lights, clock): Self::SystemData) {
        for (entity, identifier, light) in (&entities, &identifiers, &mut lights).join() {
            println!("{:#?}", eventsmanager.get_events_to_execute(identifier.0.as_str()));
            // We check the events that apply (the events that were triggered by the entities that are observed by this one)
            let events_to_execute: Vec<&Event> = eventsmanager.get_events_to_execute(identifier.0.as_str());
            for event_to_execute in events_to_execute.iter() {
                match event_to_execute {
                    Event::TrafficLightColorChange(new_color) => {
                        if new_color == &TrafficLightColor::RED {
                            light.reset_to_green();
                        }
                    }
                }
            }
            // We update the light's time (and color if applicable)
            match light.color {
                TrafficLightColor::GREEN => {
                    light.time = light.time - clock.get_dt();
                    if light.time <= (core::f64::EPSILON * S) {
                        light.reset_to_yellow();
                        eventsmanager.add_event_to_be_executed(identifier.0.as_str(), &Event::TrafficLightColorChange(TrafficLightColor::YELLOW));
                        //light.notify(&Event::TrafficLightColorChange(TrafficLightColor::YELLOW));
                    }
                },
                TrafficLightColor::YELLOW => {
                    light.time = light.time - clock.get_dt();
                    if light.time <= (core::f64::EPSILON * S) {
                        light.reset_to_red();
                        eventsmanager.add_event_to_be_executed(identifier.0.as_str(), &Event::TrafficLightColorChange(TrafficLightColor::RED))
                        //light.notify(&Event::TrafficLightColorChange(TrafficLightColor::RED));
                    }
                },
                _ => ()
            }
        }
    }
}