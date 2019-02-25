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

use ressources::{generals, clock, eventsmanager, entitytable};
use components::dynamic::{Position, Speed};
use components::statics::trafficlight::{Light, TrafficLightColor, LightUpdate, IObservable, IObserver};
use eventsmanager::{EventsManager, EventsUpdate, Observers, EventsHookUpdate};
use entitytable::{EntityTable};
use dim::si::{M, MPS, S};
use specs::prelude::*;

use crate::components::constant::{CarType, Identifier};
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
    world.add_resource(EntityTable::new());

    // Component registering
    world.register::<Identifier>();
    world.register::<Position>();
    world.register::<Speed>();
    world.register::<CarType>();
    world.register::<Light>();
    world.register::<LogRecord>();
    world.register::<Observers>();

    // Entities registering
    let vehicle1 = world
        .create_entity()
        .with(Identifier("vehicle1".to_string()))
        .with(Observers::new())
        .with(Speed { val: 2.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(CarType)
        .build();
    let vehicle2 = world
        .create_entity()
        .with(Identifier("vehicle2".to_string()))
        .with(Observers::new())
        .with(Speed { val: 4.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(CarType)
        .build();
    let vehicle3 = world
        .create_entity()
        .with(Identifier("vehicle3".to_string()))
        .with(Observers::new())
        .with(Speed { val: 1.5 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(CarType)
        .build();
    let trafficlight2 = world
        .create_entity()
        .with(Identifier("trafficlight2".to_string()))
        .with(Observers::new())
        .with(Light::new(TrafficLightColor::RED, 3.0 * S, 1.0 * S, 0.0 * S))
        .build();
    let trafficlight1 = world
        .create_entity()
        .with(Identifier("trafficlight1".to_string()))
        .with(Observers { list: vec![&trafficlight2] })
        .with(Light::new(TrafficLightColor::GREEN, 5.0 * S, 1.5 * S, 3.5 * S))
        .build();
    
    // Add every entity to the entityTable which links an entity to its Simumo id (not its Specs id)
    {
        let mut entity_table = world.write_resource::<EntityTable>();
        entity_table.insert("vehicle1".to_string(), vehicle1);
        entity_table.insert("vehicle2".to_string(), vehicle2);
        entity_table.insert("vehicle3".to_string(), vehicle3);
        entity_table.insert("trafficlight1".to_string(), trafficlight1);
        entity_table.insert("trafficlight2".to_string(), trafficlight2);
    }
    // For every entity, we define the entity it has to listen to, if any (this will be in a configuration file)
    {
        //let mut events_manager = world.write_resource::<EventsManager>();
        //events_manager.connect("trafficlight2".to_string(), "trafficlight1".to_string());
    }
    world.maintain();

    // System registering

    //NOTE :: uncomment and add a personal path to try to use the logs

    //let logger: LoggerSys<NdJsonLogger> = systems::logging::log_sys::LoggerSys::new(
    //    String::from("/home/bigjerbd/Workspace/gitlab/simumo/simulator/test"),
    //    &["CarPosition"],
    //);
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::logging::print_sys::PrintLog, "print", &[])
        .with(EventsHookUpdate, "eventshook_system", &[])
        .with(LightUpdate, "color_update", &["print"])
        .with(
            systems::physic::mobility::PositionUpdate,
            "pos_update",
            &["print"],
        )
        .with(EventsUpdate, "events_update", &["print"])
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
