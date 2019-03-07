#![allow(dead_code)]
#[macro_use]
extern crate erased_serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate specs_derive;

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
mod internal_prelude;
//mod osmgraph_api;
mod metrics;
mod ressources;
mod simulation;
mod systems;
mod topology;
mod types;
mod util;

fn main() {
    let args = command_line::CommandLineArguments::parse();
    let config = configurations::Configuration::from_path(&args.configuration_path).unwrap();

    let mut simulation = simulation::Simulation::from_config(config);
    simulation.run_simulation();
}