//todo:: remove this when program will be complete
#![allow(dead_code)]
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate simumo_derive;
#[macro_use]
extern crate specs_derive;

extern crate csv;
extern crate proc_macro2;
extern crate rand;
extern crate specs;
extern crate uuid;

mod components;
mod metrics;
mod ressources;
mod rng;
mod simulation;
mod systems;
mod topology;
mod types;
mod util;

use components::dynamic::{Position, Speed};
use ressources::*;
use specs::prelude::*;

use crate::components::constant::CarType;
use crate::components::log_record::LogRecord;
use crate::systems::clock::ClockSys;

//use crate::systems::logging::log_sys::*;
//use crate::systems::logging::loggers::ndjson_logger::NdJsonLogger;

fn main() {
    let mut world = World::new();
    //Ressources registering
    // world.add_resource(seed);
    world.add_resource(clock::Clock::new(0.25));
    world.add_resource(generals::EndTime(12.5));

    // Component registering
    world.register::<Position>();
    world.register::<Speed>();
    world.register::<CarType>();
    world.register::<LogRecord>();
    world
        .create_entity()
        .with(Speed(2.0))
        .with(Position { x: 0.0, y: 0.0 })
        .with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed(4.0))
        .with(Position { x: 0.0, y: 0.0 })
        .with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed(1.5))
        .with(Position { x: 0.0, y: 0.0 })
        .with(CarType)
        .build();

    // System registering

    //NOTE :: uncomment and add a personal path to try to use the logs

    //let logger: LoggerSys<NdJsonLogger> = systems::logging::log_sys::LoggerSys::new(
    //    String::from("/home/bigjerbd/Workspace/gitlab/simumo/simulator/test"),
    //    &["CarPosition"],
    //);
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::logging::print_sys::PrintLog, "print", &[])
        .with(
            systems::physic::mobility::PositionUpdate,
            "pos_update",
            &["print"],
        )
        // NOTE uncomment this also
        //.with(
        //    systems::recording::car_pos_recorder::CarPosRec::new(0.5),
        //    "log_car",
        //    &["pos_update"],
        //)
        //.with(logger, "logger_sys", &["log_car"])
        //.with_barrier()
        .with(ClockSys, "clock_sys", &[])
        .build();
    dispatcher.setup(&mut world.res);

    // Game Loop
    loop {
        dispatcher.dispatch(&mut world.res);
        // Maintain dynamically add and remove entities in dispatch.
        world.maintain();
        // verify if the simulation is overs
        let clock = world.read_resource::<clock::Clock>();
        let end_time = world.read_resource::<generals::EndTime>();
        if clock.get_time() >= end_time.0 {
            break;
        }
    }
    println!("Showing results log...");
    //sim.get_state();
}
