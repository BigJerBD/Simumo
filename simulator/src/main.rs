//todo:: remove this when program will be complete
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

// Débugger graphique
mod vdebugger;
use vdebugger::{graphics_00, input};
use graphics_00::{ DrawClear, DrawRectangles, ApplySpin, register_spin_rect };
use input::ArrowKeysPressed;
use input::KeyboardSystem;
extern crate piston_window;
extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

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
use dim::si::{M, MPS, S};
use specs::prelude::*;

use crate::components::constant::CarType;
use crate::components::log_record::LogRecord;
use crate::systems::clock::ClockSys;

use crate::systems::logging::log_sys::*;
use crate::systems::logging::loggers::ndjson_logger::NdJsonLogger;

fn main() {
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

    let mut update_dispatcher = DispatcherBuilder::new()
        .with(ApplySpin, "apply_spin", &[])
        .with(KeyboardSystem, "keyboard_system", &[])
        .build();
    update_dispatcher.setup(&mut world.res);
    register_spin_rect(&mut world);
    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        let mut clock_time;
        let mut max_time;
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

        let mut arrow_keys = ArrowKeysPressed { up: false, left: false, right: false, down: false };
        if let Some(r) = e.render_args() {
            world.add_resource(r);
            render_dispatcher.dispatch(&mut world.res);
            world.maintain();
        }
    }
    println!("Showing results log...");
}

fn setup_graphics(world: &mut World) -> (Dispatcher<'static, 'static>, Window) {
    let opengl = OpenGL::V3_2;

    let window: Window = WindowSettings::new(
            "spinning-square",
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