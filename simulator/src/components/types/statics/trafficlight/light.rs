extern crate specs;
use specs::prelude::*;
use typeinfo::TypeInfo;
use typeinfo_derive::*;
use crate::metrics::Fdim;
use dim::si::{S, Second};

#[derive(Copy, Clone, Debug, Serialize, PartialEq)]
pub enum TrafficLightColor { RED, YELLOW, GREEN }

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
    pub fn reset_to_green(&mut self) {
        self.color = TrafficLightColor::GREEN;
        self.time = self.max_green_time;
    }
    pub fn reset_to_yellow(&mut self) {
        self.color = TrafficLightColor::YELLOW;
        self.time = self.max_yellow_time;
    }
    pub fn reset_to_red(&mut self) {
        self.color = TrafficLightColor::RED;
        self.time = 0.0 * S;
    }
}