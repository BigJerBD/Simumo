use specs::prelude::*;

use crate::components::dynamic::*;
use crate::ressources::Clock;

pub struct PositionUpdate;

impl<'a> System<'a> for PositionUpdate {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Speed>, Read<'a, Clock>);

    fn run(&mut self, (mut pos, vel, clock): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.val * clock.dt;
            pos.y += vel.val * clock.dt;
        }
    }
}

pub struct SpeedUpdate;

impl<'a> System<'a> for SpeedUpdate {
    type SystemData = (WriteStorage<'a, Speed>, ReadStorage<'a, Acceleration>, Read<'a, Clock>);

    fn run(&mut self, (mut vel, acc, clock): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            vel.val += acc.val * clock.dt;
        }
    }
}
