use crate::ressources::{clock};
use crate::eventsmanager::{Event, EventsUpdate};

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

#[derive(TypeInfo, Debug)]
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


impl Component for Light {
    type Storage = FlaggedStorage<Self>;
}

#[derive(Default)]
pub struct SysA {
    reader_id: Option<ReaderId<ComponentEvent>>,
    inserted: BitSet,
    modified: BitSet,
    removed: BitSet,
}

impl<'a> System<'a> for SysA {
    type SystemData = (Entities<'a>, ReadStorage<'a, Light>);

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.reader_id = Some(WriteStorage::<Light>::fetch(&res).register_reader());
    }

    fn run(&mut self, (entities, tracked): Self::SystemData) {
        self.modified.clear();
        self.inserted.clear();
        self.removed.clear();

        let events = tracked.channel().read(self.reader_id.as_mut().expect("ReaderId not found"));
        for event in events {
            match event {
                ComponentEvent::Modified(id) => {
                    self.modified.add(*id);
                }
                ComponentEvent::Inserted(id) => {
                    self.inserted.add(*id);
                }
                ComponentEvent::Removed(id) => {
                    self.removed.add(*id);
                }
            }
        }
        for (entity, _tracked, _) in (&entities, &tracked, &self.modified).join() {
            println!("modified: {:?}", entity);
        }
        for (entity, _tracked, _) in (&entities, &tracked, &self.inserted).join() {
            println!("inserted: {:?}", entity);
        }
        for (entity, _tracked, _) in (&entities, &tracked, &self.removed).join() {
            println!("removed: {:?}", entity);
        }
    }
}

#[derive(Default)]
pub struct SysB;
impl<'a> System<'a> for SysB {
    type SystemData = (Entities<'a>, WriteStorage<'a, Light>);
    fn run(&mut self, (entities, mut tracked): Self::SystemData) {
        for (entity, mut restricted) in (&entities, &mut tracked.restrict_mut()).join() {
            if entity.id() % 2 == 0 {
                let mut comp = restricted.get_mut_unchecked();
                //comp.color = TrafficLightColor::ORANGE;
            }
        }
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
        Entities<'a>,
        WriteStorage<'a, Light>,
        Read<'a, clock::Clock>
    );

    fn run(&mut self, (entities, mut lights, clock): Self::SystemData) {
        let mut x: &mut Light = &mut Light::new(TrafficLightColor::GREEN, 3.0 * S, 1.0 * S, 0.0 * S);
        for (entity, light) in (&entities, &mut lights).join() {
            if (light.color == TrafficLightColor::RED) {
                x = light;
            } else {
                match light.color {
                    TrafficLightColor::GREEN => {
                        light.time = light.time - clock.get_dt();
                        if light.time <= (core::f64::EPSILON * S) {
                            light.reset_to_yellow();
                            light.notify(&Event::TrafficLightColorChange(TrafficLightColor::YELLOW));
                        }
                    },
                    TrafficLightColor::YELLOW => {
                        light.time = light.time - clock.get_dt();
                        if light.time <= (core::f64::EPSILON * S) {
                            light.reset_to_red();
                            light.notify(&Event::TrafficLightColorChange(TrafficLightColor::RED));
                        }
                    },
                    TrafficLightColor::RED => {
                        x.reset_to_green();
                    },
                    _ => ()
                }
            }
        }
    }
}