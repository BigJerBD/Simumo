use specs::prelude::{Component, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct EnergyControl(pub i32);
