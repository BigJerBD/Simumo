use crate::ressources::{clock};

use specs::prelude::*;

use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Clone, Debug, Serialize)]
pub enum TrafficLightColor { RED, YELLOW, GREEN }

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct Color(pub TrafficLightColor);

pub struct ColorUpdate;
impl<'a> System<'a> for ColorUpdate {
    type SystemData = (WriteStorage<'a, Color>, ReadStorage<'a, GreenTime>);

    fn run(&mut self, (mut colors, green_times): Self::SystemData) {
        for (color, green_time) in (&mut colors, &green_times).join() {
            if green_time.0 == 0.0 {
                color.0 = TrafficLightColor::RED;
            } else {
                color.0 = TrafficLightColor::GREEN;
            }
        }
    }
}

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct MaxGreenTime(pub f64);

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct GreenTime(pub f64);

pub struct GreenTimeUpdate;
impl<'a> System<'a> for GreenTimeUpdate {
    type SystemData = (
        WriteStorage<'a, GreenTime>,
        ReadStorage<'a, MaxGreenTime>,
        Read<'a, clock::Clock>
    );

    fn run(&mut self, (mut green_times, max_green_times, clock): Self::SystemData) {
        for (green_time, max_green_time) in (&mut green_times, &max_green_times).join() {
            let new_green_time = green_time.0 - clock.get_dt();
            if new_green_time > core::f64::EPSILON {
                green_time.0 = new_green_time;
            } else {
                green_time.0 = 0.0;
            }
        }
    }
}