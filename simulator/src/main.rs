#![allow(dead_code)]
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate simumo_derive;
extern crate dimensioned as dim;


mod topology;

mod types;

mod components;
mod metrics;
mod ressources;
mod simulation;
mod systems;
mod util;

use ressources::*;

use components::dynamic::{Position, Speed};
use components::statics::trafficlight::{Light, TrafficLightColor, LightUpdate, IObservable, IObserver};
use eventsmanager::{EventsManager, EventsUpdate, Event};
use dim::si::{M, MPS, S};
use specs::prelude::*;

use crate::components::constant::CarType;
use crate::components::log_record::LogRecord;
use crate::systems::clock::ClockSys;

//use crate::systems::logging::log_sys::*;
//use crate::systems::logging::loggers::ndjson_logger::NdJsonLogger;

fn main() {
    let mut world = World::new();

    //Ressources registering
    world.add_resource(clock::Clock::new(0.25 * S));
    world.add_resource(generals::EndTime { val: 12.5 * S });
    world.add_resource(generals::LogDirectory { val: String::from("testpath") });
    world.add_resource(EventsManager::new());

    // Component registering
    world.register::<Position>();
    world.register::<Speed>();
    world.register::<CarType>();
    world.register::<Light>();
    world.register::<LogRecord>();
    world
        .create_entity()
        .with(Speed { val: 2.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 4.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 1.5 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(CarType)
        .build();
    let mut green = Light::new(TrafficLightColor::GREEN, 5.0 * S, 1.5 * S, 3.5 * S);
    let red = Light::new(TrafficLightColor::RED, 3.0 * S, 1.0 * S, 0.0 * S);
    {
        let mut eventsManager = world.write_resource::<EventsManager>();
        //eventsManager.connect(&mut green, &red);
        //eventsManager.connect(0, 1);
        &red.subscribe(&mut green);
    }
    world
        .create_entity()
        .with(green)
        .build();
    world
        .create_entity()
        .with(red)
        .build();

    
    
    //red.subscribe(&green);

    // System registering

    //NOTE :: uncomment and add a personal path to try to use the logs

    //let logger: LoggerSys<NdJsonLogger> = systems::logging::log_sys::LoggerSys::new(
    //    String::from("/home/bigjerbd/Workspace/gitlab/simumo/simulator/test"),
    //    &["CarPosition"],
    //);
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::logging::print_sys::PrintLog, "print", &[])
        //.with(EventsUpdate, "events_update", &["print"])
        .with(LightUpdate, "color_update", &["print"])
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
        // verify if the simulation is over
        let clock = world.read_resource::<clock::Clock>();
        let end_time = world.read_resource::<generals::EndTime>();
        if clock.get_time() >= end_time.val {
            break;
        }
    }
    println!("Showing results log...");
    //sim.get_state();
}
