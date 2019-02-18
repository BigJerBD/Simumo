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
    type SystemData = (
        WriteStorage<'a, Color>,
        WriteStorage<'a, Time>,
        ReadStorage<'a, MaxGreenTime>,
        ReadStorage<'a, MaxYellowTime>,
        Read<'a, clock::Clock>
    );

    fn run(&mut self, (mut colors, mut times, max_green_times, max_yellow_times, clock): Self::SystemData) {
        for (color, time, max_green_time, max_yellow_time) in
            (&mut colors, &mut times, &max_green_times, &max_yellow_times).join()
        {
            
            match color.0 {
                TrafficLightColor::GREEN => {
                    time.0 = time.0 - clock.get_dt();
                    if time.0 <= core::f64::EPSILON {
                        time.0 = max_yellow_time.0;
                        color.0 = TrafficLightColor::YELLOW;
                    }
                },
                TrafficLightColor::YELLOW => {
                    time.0 = time.0 - clock.get_dt();
                    if time.0 <= core::f64::EPSILON {
                        time.0 = 0.0;
                        color.0 = TrafficLightColor::RED;
                    }
                },
                TrafficLightColor::RED => {

                }
            }
        }
    }
}

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct MaxGreenTime(pub f64);

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct MaxYellowTime(pub f64);

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct Time(pub f64);

pub struct GreenTimeUpdate;
impl<'a> System<'a> for GreenTimeUpdate {
    type SystemData = (
        WriteStorage<'a, Time>,
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