#![allow(dead_code)]
#![allow(clippy::type_complexity)]
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate specs_derive;
#[macro_use]
extern crate log;
extern crate env_logger;

extern crate argparse;
extern crate csv;
extern crate dimensioned as dim;
extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate proc_macro2;
extern crate rand;
extern crate serde;
extern crate simumo_derive;
extern crate specs;
extern crate typeinfo;
extern crate typeinfo_derive;
extern crate uuid;

mod command_line;
mod components;
mod configurations;
mod entities;
mod errors;
mod metrics;
mod osmgraph_api;
mod ressources;
mod simulation;
mod systems;

fn main() {
    env_logger::init();
    let args = command_line::CommandLineArguments::parse();
    let config = configurations::Configuration::from_yaml(&args.configuration_path).unwrap();

    let mut simulation = simulation::Simulation::from_config(config);
    simulation.run_simulation();
}
