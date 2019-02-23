use std::cell::RefCell;
use std::sync::Mutex;

use crate::metrics::Fdim;

lazy_static! {
    pub static ref M_LOG_DIRECTORY: Mutex<RefCell<String>> =
        Mutex::new(RefCell::new(String::new()));
    pub static ref LOG_DIRECTORY: String = M_LOG_DIRECTORY.lock().unwrap().borrow().clone();
}

use dim::si::{Second, MIN};

#[derive(Default)]
pub struct LogDirectory {
    pub val: String,
}

pub struct EndTime {
    pub val: Second<Fdim>,
}

impl Default for EndTime {
    fn default() -> Self {
        Self { val: MIN }
    }
}
