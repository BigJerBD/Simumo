mod clock;

use clock::*;

fn main() {
    let mut simend = false;
    let mut simclock: Clock = Clock::new(0.025);
    // initialiser les variables de l'état du système ici
    // créer la liste des évènements ici
    while !simend {
        simclock.update();
        // faire les évènements
        // update stats
        println!("{}", simclock.get_time());
        if simclock.get_time() >= 5. {
            simend = true;
        }
    }
}