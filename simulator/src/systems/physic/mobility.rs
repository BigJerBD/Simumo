use specs::prelude::*;

use crate::components::constant::*;
use crate::components::dynamic::*;

pub struct PositionUpdate;
impl<'a> System<'a> for PositionUpdate {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Speed>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.0;
            pos.y += vel.0;
        }
    }
}

pub struct SpeedUpdate;
impl<'a> System<'a> for SpeedUpdate {
    type SystemData = (WriteStorage<'a, Speed>, ReadStorage<'a, Acceleration>);

    fn run(&mut self, (mut vel, acc): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            vel.0 += acc.0;
        }
    }
}
