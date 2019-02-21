//todo:: remove this when program will be complete
#![allow(dead_code)]
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate specs_derive;
extern crate typeinfo_derive;
extern crate typeinfo;


use std::any::Any;

use typeinfo::*;
use typeinfo_derive::*;
use specs::prelude::*;

use ressources::*;

use crate::systems::clock::StandardClockSys;
use crate::systems::recorders::car_pos_recorder::CarPosRec;
use crate::components::controls::EnergyControl;

mod topology;
mod types;
mod components;
mod metrics;
mod ressources;
mod simulator;
mod systems;
mod util;
mod internal_prelude;
mod errors;




fn main() {
    let mut world = World::new();
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
        .with(StandardClockSys, "clock_sys", &[])
        .build();
    dispatcher.setup(&mut world.res);


    world.add_resource(clock::Clock::new(0.25));
    world.add_resource(generals::EndTime(12.5));
    world.add_resource(generals::LogDirectory(String::from("testpath")));

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
    let mut data : Vec<Box<&Any>> = Vec::new();
    data.push(
        Box::from(
            &StandardClockSys as &Any
        )
    );
    data.push(
        Box::from(
            &CarPosRec::default() as &Any
        )
    );




    // Game Loop
    loop {
        dispatcher.dispatch(&mut world.res);
        // Maintain dynamically add and remove entities in dispatch.
        world.maintain();
        // verify if the simulator is overs
        let clock = world.read_resource::<clock::Clock>();
        let end_time = world.read_resource::<generals::EndTime>();
        if clock.get_time() >= end_time.0 {
            break;
        }
    }
    println!("Showing results log...");
    //sim.get_state();
}
