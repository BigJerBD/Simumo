#[macro_use]
extern crate specs_derive;
extern crate specs;

mod simulation;
mod topology;
mod types;

mod components;
mod metrics;
mod ressources;
mod systems;
mod util;

use simulation::*;
use topology::Topology;

use components::dynamic::{Position, Speed};
use specs::prelude::*;

fn main() {
    let mut sim: Simulation = Simulation::new(0.25, 12.5);

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Speed>();

    world
        .create_entity()
        .with(Speed(2.0))
        .with(Position { x: 0.0, y: 0.0 })
        .build();
    world
        .create_entity()
        .with(Speed(4.0))
        .with(Position { x: 0.0, y: 0.0 })
        .build();
    world
        .create_entity()
        .with(Speed(1.5))
        .with(Position { x: 0.0, y: 0.0 })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::physic::print_system::PrintLog, "print", &[])
        .with(
            systems::physic::movement_system::PositionUpdate,
            "pos_update",
            &["print"],
        )
        .build();
    dispatcher.setup(&mut world.res);

    while !sim.has_ended() {
        sim.update_clock();
        sim.execute_events();
        sim.update_objects();

        dispatcher.dispatch(&mut world.res);

        println!("Simulation time: {}", sim.get_time());
    }
    println!("Showing results log...");
    sim.get_state();
}
