extern crate specs;
#[macro_use]
extern crate specs_derive;

mod simulation;
mod topology;
mod types;

mod components;
mod systems;
mod metrics;
mod ressources;


use simulation::*;
use topology::Topology;



fn main() {
    let mut sim: Simulation = Simulation::new(0.25, 12.5);


    while !sim.has_ended() {
        sim.update_clock();
        sim.execute_events();
        sim.update_objects();


        println!("Simulation time: {}", sim.get_time());

    }
    println!("Showing results log...");
    sim.get_state();
    let topology = Topology::new();
    println!("{:#?}", topology);
}
