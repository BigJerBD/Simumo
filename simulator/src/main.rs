extern crate core;

mod simulation;
mod topology;
mod types;

use simulation::*;
use topology::Topology;

mod components;

use components::generics::{Component,TopologyState};
use components::agents::example_agents::*;
use components::systems::example_system::*;

fn main() {
    let mut sim: Simulation = Simulation::new(0.25, 12.5);

    //adding a example component in the simulation
    let mut comp: Component<ExampleAgent, ExampleSystem,
                            ExampleControls, ExampleProperties> =
        Component::new(
            1,
            ExampleSystem::new(),
            ExampleAgent {},
        );

    while !sim.has_ended() {
        sim.update_clock();
        sim.execute_events();
        sim.update_objects();

        //added example_object updating
        comp.chose_action(&TopologyState {});
        comp.update();
        let id = comp.get_id();
        let (x,y) = &comp.get_properties().position;
        println!("position of comp #{}: ({},{})",id,x,y);

        println!("Simulation time: {}", sim.get_time());

    }
    println!("Showing results log...");
    sim.get_state();
    let topology = Topology::new();
    println!("{:#?}", topology);
}
