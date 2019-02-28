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
extern crate specs_derive;

extern crate csv;
extern crate dimensioned as dim;
extern crate proc_macro2;
extern crate rand;
extern crate serde;
extern crate simumo_derive;
extern crate specs;
extern crate typeinfo;
extern crate typeinfo_derive;
extern crate uuid;
extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;


// Débugger graphique
use vdebugger::graphics::{ DrawClear, DrawRectangles, Rectangle };
use std::process::Command;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

mod vdebugger;
mod topology;
mod command_line;
mod types;

mod components;
mod configurations;
mod errors;
mod internal_prelude;
mod metrics;
mod ressources;
mod rng;
mod simulation;
mod systems;
mod topology;
mod types;
mod util;

use configurations::Configuration;
use ressources::*;

use components::dynamic::{Position, Speed};
use dim::si::{M, MPS, S};
use specs::prelude::*;

use crate::components::constant::CarType;
use crate::components::log_record::LogRecord;
use crate::systems::clock::ClockSys;

use crate::systems::logging::log_sys::*;
use crate::systems::logging::loggers::ndjson_logger::NdJsonLogger;

fn main() {

    //let args = command_line::CommandLineArguments::parse();
    //let config = Configuration::from_path(&args.configuration_path).unwrap();
    //config.setup();
    //
    //if args.verbose {}



    let mut world = World::new();

    let (mut render_dispatcher, mut window) = setup_graphics(&mut world);

    //Ressources registering
    world.add_resource(clock::Clock::new(0.25 * S));
    world.add_resource(generals::EndTime { val: 12.5 * S });
    world.add_resource(generals::LogDirectory { val: String::from("testpath") });

    // Component registering
    world.register::<Position>();
    world.register::<Speed>();
    world.register::<CarType>();
    world.register::<LogRecord>();
    world
        .create_entity()
        .with(Speed { val: 2.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(Rectangle { width: 5.0, height: 5.0 })
        .with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 4.0 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(Rectangle { width: 5.0, height: 5.0 })
        .with(CarType)
        .build();
    world
        .create_entity()
        .with(Speed { val: 1.5 * MPS })
        .with(Position { x: 0.0 * M, y: 0.0 * M })
        .with(Rectangle { width: 5.0, height: 5.0 })
        .with(CarType)
        .build();

    // System registering

    //NOTE :: uncomment and add a personal path to try to use the logs

    let logger: LoggerSys<NdJsonLogger> = systems::logging::log_sys::LoggerSys::new(
        String::from("C:/Users/Utilisateur/Desktop/UdeS/Projet S6/simumo/simulator/src"),
        &["CarPosition"],
    );
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::logging::print_sys::PrintLog, "print", &[])
        .with(systems::physic::mobility::PositionUpdate, "pos_update", &["print"])
        .with(systems::recording::car_pos_recorder::CarPosRec::new(0.5), "log_car", &["pos_update"])
        .with(logger, "logger_sys", &["log_car"])
        .with_barrier()
        .with(ClockSys, "clock_sys", &[])
        .build();
    dispatcher.setup(&mut world.res);

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        let clock_time;
        let max_time;
        {
            let clock = world.read_resource::<clock::Clock>();
            clock_time = clock.get_time();
            let end_time = world.read_resource::<generals::EndTime>();
            max_time = end_time.val;
        }

        if clock_time < max_time {
            dispatcher.dispatch(&mut world.res);
            world.maintain();
        }
        if let Some(r) = e.render_args() {
            world.add_resource(r);
            render_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }

        // Wait 0.2s so we can see the changes on the visual debugger
        let mut child = Command::new("sleep").arg("0.05").spawn().unwrap();
        let _result = child.wait().unwrap();
    }
    println!("Showing results log...");
}

fn setup_graphics(world: &mut World) -> (Dispatcher<'static, 'static>, Window) {
    let opengl = OpenGL::V3_2;
    let window: Window = WindowSettings::new(
            "Simumo - Visual debugger",
            [1440, 2800]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let graphics_handle = GlGraphics::new(opengl);

    let mut dispatcher = DispatcherBuilder::new()
        .with_thread_local(DrawClear)
        .with_thread_local(DrawRectangles)
        .build();

    world.add_resource(graphics_handle);

    dispatcher.setup(&mut world.res);

    (dispatcher, window)
}