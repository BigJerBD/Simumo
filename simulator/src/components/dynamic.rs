use specs::prelude::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Angle(pub f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Speed(pub f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Acceleration(pub f32);
