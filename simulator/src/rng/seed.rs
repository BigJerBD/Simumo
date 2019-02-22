use std::cell::Cell;
use std::sync::Mutex;
use uuid::Uuid;

//This enable no-mutability and static.
lazy_static! {
    pub static ref M_SEED: Mutex<Cell<Uuid>> = Mutex::new(Cell::new(Uuid::nil()));
    pub static ref SEED: Uuid = M_SEED.lock().unwrap().get();
}
