use crate::components::dynamic::Position;
use crate::components::dynamic::Speed;
use crate::components::statics::trafficlight::Light;
use crate::components::statics::trafficlight::TrafficLightColor;
use specs::world::Builder;
use specs::World;

use crate::components::constant::Identifier;
use crate::components::constant::Rectangle;

use dim::si::{M, MPS, S};

pub fn create_entities(world: &mut World) {
    world
        .create_entity()
        .with(Speed { val: 2.0 * MPS })
        .with(Position {
            x: 0.0 * M,
            y: 0.0 * M,
        })
        .with(Rectangle {
            width: 5.0,
            height: 5.0,
        })
        //.with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 4.0 * MPS })
        .with(Position {
            x: 0.0 * M,
            y: 0.0 * M,
        })
        .with(Rectangle {
            width: 5.0,
            height: 5.0,
        })
        //.with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 1.5 * MPS })
        .with(Position {
            x: 0.0 * M,
            y: 0.0 * M,
        })
        .with(Rectangle {
            width: 5.0,
            height: 5.0,
        })
        //.with(CarType)
        .build();
    world
        .create_entity()
        .with(Identifier("trafficlight1".to_string()))
        .with(Light::new(TrafficLightColor::GREEN, 5.0 * S, 1.5 * S, 3.5 * S))
        .build();
    world
        .create_entity()
        .with(Identifier("trafficlight2".to_string()))
        .with(Light::new(TrafficLightColor::RED, 3.0 * S, 1.0 * S, 0.0 * S))
        .build();
}
