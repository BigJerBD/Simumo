use crate::components::type_prelude::*;

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Length(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Mass(pub f32);

#[simucomponent_data]
#[storage(VecStorage)]
pub struct Identifier(pub i32);

//entity types
#[simucomponent_tag]
#[storage(VecStorage)]
pub struct CarType;

#[simucomponent_tag]
#[storage(VecStorage)]
pub struct BikeType;
