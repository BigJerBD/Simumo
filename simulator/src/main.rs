//todo:: remove this when program will be complete
#![allow(dead_code)]
extern crate argparse;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate simumo_derive;
#[macro_use]
extern crate specs_derive;

extern crate csv;
extern crate dimensioned as dim;
extern crate proc_macro2;
extern crate rand;
extern crate serde;
extern crate specs;
extern crate typeinfo;
extern crate typeinfo_derive;
extern crate uuid;

mod command_line;
mod components;
mod configurations;
mod metrics;
mod ressources;
mod rng;
mod simulation;


use std::any::Any;

use dim::si::S;
use ressources::*;
use specs::prelude::*;
use crate::systems::clock::StandardClockSystem;
use crate::systems::recorders::car_pos_rec_sys::CarPosRecSystem;


mod components;
mod errors;
mod internal_prelude;
mod metrics;
mod ressources;
mod simulator;
mod systems;
mod topology;
mod types;
mod util;

fn main() {
    let mut world = World::new();
    command_line::arguments::execute_arguments(); //Find better function name.

    //print!("{:?}", EnergyControl::type_name());
    //Ressources registering
    // System registering

    //NOTE :: uncomment and add a personal path to try to use the logs

    //let logger: LoggerSys<NdJsonLogger> = systems::types::log_sys::LoggerSys::new(
    //    String::from("/home/bigjerbd/Workspace/gitlab/simumo/simulator/test"),
    //    &["CarPosition"],
    //);
    let mut dispatcher = DispatcherBuilder::new()
        //.with(systems::loggers::print_sys::PrintLog, "print", &[])
        //.with(
        //    systems::physic::acceleration_sys::PositionUpdate,
        //    "pos_update",
        //    &["print"],
        //)
        // NOTE uncomment this also
        //.with(
        //    systems::recorders::car_pos_recorder::CarPosRec::new(0.5),
        //    "log_car",
        //    &["pos_update"],
        //)
        //.with(logger, "logger_sys", &["log_car"])
        //.with_barrier()
        .with(StandardClockSystem, "clock_sys", &[])
        .build();
    dispatcher.setup(&mut world.res);

    world.add_resource(clock::Clock::new(0.25 * S));
    world.add_resource(generals::EndTime { val: 12.5 * S });
    world.add_resource(generals::LogDirectory {
        val: String::from("testpath"),
    });

    world
        .create_entity()
        //.with(Speed(2.0))
        //.with(Position { x: 0.0, y: 0.0 })
        //.with(CarType)
        .build();
    world
        .create_entity()
        //.with(Speed(4.0))
        //.with(Position { x: 0.0, y: 0.0 })
        //.with(CarType)
        .build();
    world
        .create_entity()
        //.with(Speed(1.5))
        //.with(Position { x: 0.0, y: 0.0 })
        //.with(CarType)
        .build();

    //let test = &StandardClockSys as &Test;
    //let test2 = test.downcast_ref::<&StandardClockSys>();
    let mut data: Vec<Box<&Any>> = Vec::new();
    data.push(Box::from(&StandardClockSystem as &Any));
    data.push(Box::from(&CarPosRecSystem::default() as &Any));

    // Game Loop
    loop {
        dispatcher.dispatch(&mut world.res);
        // Maintain dynamically add and remove entities in dispatch.
        world.maintain();
        // verify if the simulator is overs
        let clock = world.read_resource::<clock::Clock>();
        let end_time = world.read_resource::<generals::EndTime>();
        if clock.get_time() >= end_time.val {
            break;
        }
    }
    println!("Showing results log...");
    //sim.get_state();
}
