mod simulation;
use simulation::*;

fn main() {
    let mut sim: Simulation = Simulation::new(0.025, 12.5);
    // initialiser les variables de l'état du système ici
    // créer la liste des évènements ici
    while !sim.has_ended() {
        sim.update_clock();
        sim.execute_events();
        sim.update_objects();
        println!("Simulation time: {}s", sim.get_time());
    }
    println!("Showing results log...");
    sim.get_state();
}