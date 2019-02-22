use std::cell::RefCell;
use std::sync::Mutex;

lazy_static! {
    pub static ref M_LOG_DIRECTORY: Mutex<RefCell<String>> = Mutex::new(RefCell::new(String::new()));
    pub static ref LOG_DIRECTORY: String = M_LOG_DIRECTORY.lock().unwrap().borrow().clone();
}

#[derive(Default)]
pub struct EndTime(pub f64);
