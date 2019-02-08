use specs::prelude::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Length(pub f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Mass(pub f32);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Identifier(pub i32);

//entity types
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct CarType;
pub struct BikeType;
