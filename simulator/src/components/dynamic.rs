use specs::prelude::{Component, VecStorage};

use typeinfo::TypeInfo;
use typeinfo_derive::*;

#[derive(Component, TypeInfo, Clone, Debug, Serialize)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Serialize, TypeInfo)]
#[storage(VecStorage)]
pub struct Angle(pub f32);

#[derive(Component, Debug, Serialize, TypeInfo)]
#[storage(VecStorage)]
pub struct Speed(pub f32);

#[derive(Component, Debug, Serialize, TypeInfo)]
#[storage(VecStorage)]
pub struct Acceleration(pub f32);
