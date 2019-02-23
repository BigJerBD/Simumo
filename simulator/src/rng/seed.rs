use std::cell::Cell;
use std::sync::Mutex;
use uuid::Uuid;

//TODO: DON'T USE THIS ANYMORE. Maudite Cohonnerie
lazy_static! {
    pub static ref M_SEED: Mutex<Cell<Uuid>> = Mutex::new(Cell::new(Uuid::nil()));
    pub static ref SEED: Uuid = {
        if M_SEED.lock().unwrap().get().is_nil() {
            panic!("cannot get unsetted seed.");
        }
        M_SEED.lock().unwrap().get()
    };
}
