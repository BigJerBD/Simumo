use crate::components::type_prelude::*;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Angle(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Speed(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Acceleration(pub f32);
