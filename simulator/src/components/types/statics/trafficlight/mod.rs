use crate::ressources::{clock};
use crate::eventsmanager::{Event, EventsManager};
use crate::{Identifier};

use dim::si::{S, Second};
use serde::ser::{Serialize, Serializer};
use simumo_derive::*;
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use crate::components::simumo_component::*;
use crate::metrics::Fdim;

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
}

impl Light {
    pub fn new(color: TrafficLightColor, max_green_time: Second<Fdim>, max_yellow_time: Second<Fdim>, time: Second<Fdim>) -> Self {
        Self {
            color,
            max_green_time,
            max_yellow_time,
            time
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

pub struct LightUpdate;
impl<'a> System<'a> for LightUpdate {
    type SystemData = (
        Write<'a, EventsManager>,
        ReadStorage<'a, Identifier>,
        WriteStorage<'a, Light>,
        Read<'a, clock::Clock>
    );

    fn run(&mut self, (mut eventsmanager, identifiers, mut lights, clock): Self::SystemData) {
        for (identifier, light) in (&identifiers, &mut lights).join() {
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
                    }
                },
                TrafficLightColor::YELLOW => {
                    light.time = light.time - clock.get_dt();
                    if light.time <= (core::f64::EPSILON * S) {
                        light.reset_to_red();
                        eventsmanager.add_event_to_be_executed(identifier.0.as_str(), &Event::TrafficLightColorChange(TrafficLightColor::RED))
                    }
                },
                _ => ()
            }
        }
    }
}