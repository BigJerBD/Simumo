use specs::World;
use specs::world::Builder;
use crate::components::dynamic::Speed;
use crate::components::dynamic::Position;

use crate::components::constant::CarType;
use crate::components::constant::Rectangle;

use dim::si::{S,M,MPS};

pub fn create_entities(world: &mut  World){
    world
        .create_entity()
        .with(Speed { val: 2.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(Rectangle { width: 5.0, height: 5.0 })
        //.with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 4.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(Rectangle { width: 5.0, height: 5.0 })
        //.with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 1.5 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(Rectangle { width: 5.0, height: 5.0 })
        //.with(CarType)
        .build();
}