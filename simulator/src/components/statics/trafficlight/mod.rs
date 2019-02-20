use crate::ressources::{clock};
use crate::eventsmanager::{Event, EventsUpdate};

use specs::prelude::*;

use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Copy, Clone, Debug, Serialize, PartialEq)]
pub enum TrafficLightColor { RED, YELLOW, GREEN }

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct Light {
    pub color: TrafficLightColor,
    pub max_green_time: f64,
    pub max_yellow_time: f64,
    pub time: f64,
    pub observers: Vec<LightObserver>
}

impl Light {
    pub fn new(color: TrafficLightColor, max_green_time: f64, max_yellow_time: f64, time: f64) -> Self {
        Self {
            color,
            max_green_time,
            max_yellow_time,
            time,
            observers: Vec::new()
        }
    }
    fn resetToGreen(&mut self) {
        self.color = TrafficLightColor::GREEN;
        self.time = self.max_green_time;
    }
    fn resetToYellow(&mut self) {
        self.color = TrafficLightColor::YELLOW;
        self.time = self.max_yellow_time;
    }
    fn resetToRed(&mut self) {
        self.color = TrafficLightColor::RED;
        self.time = 0.0;
    }
}

#[derive(Clone, Debug, Serialize)]
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
        self.observers.push(LightObserver::Light(observer.clone()));
    }
    fn notify(&self, event: &Event) {
        for observer in self.observers.iter() {
            match observer.clone() {
                LightObserver::Light(mut light) => light.update(self, event)
            }
        }
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
                    self.resetToGreen();
                }
            }
        }
    }
}

pub struct LightUpdate;
impl<'a> System<'a> for LightUpdate {
    type SystemData = (
        WriteStorage<'a, Light>,
        Read<'a, clock::Clock>
    );

    fn run(&mut self, (mut lights, clock): Self::SystemData) {
        for light in (&mut lights).join() {
            match light.color {
                TrafficLightColor::GREEN => {
                    light.time = light.time - clock.get_dt();
                    if light.time <= core::f64::EPSILON {
                        light.resetToYellow();
                        light.notify(&Event::TrafficLightColorChange(TrafficLightColor::YELLOW));
                    }
                },
                TrafficLightColor::YELLOW => {
                    light.time = light.time - clock.get_dt();
                    if light.time <= core::f64::EPSILON {
                        light.resetToRed();
                        light.notify(&Event::TrafficLightColorChange(TrafficLightColor::RED));
                    }
                },
                TrafficLightColor::RED => {
                    
                }
            }
        }
    }
}