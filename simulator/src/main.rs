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
mod topology;
mod types;

mod metrics;
mod rng;
mod components;
mod configurations;
mod entities;
mod errors;
mod internal_prelude;
//mod osmgraph_api;
mod ressources;
mod simulation;
mod systems;
mod util;

fn main() {
    //let args = command_line::CommandLineArguments::parse();
    //let config = Configuration::from_path(&args.configuration_path).unwrap();
    //config.setup();
    //
    //if args.verbose {}

    let mut simulation = simulation::Simulation::new();
    simulation.run_simulation();
}