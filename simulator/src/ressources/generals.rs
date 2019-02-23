use crate::internal_prelude::*;
use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

use dim::si::{Second, MIN};
use std::cell::RefCell;
use std::sync::Mutex;

lazy_static! {
    pub static ref M_LOG_DIRECTORY: Mutex<RefCell<String>> =
        Mutex::new(RefCell::new(String::new()));
    pub static ref LOG_DIRECTORY: String = M_LOG_DIRECTORY.lock().unwrap().borrow().clone();
}

#[simuresource]
pub struct LogDirectory {
    pub val: String,
}

#[derive(Deserialize)]
pub struct EndTime {
    #[serde(deserialize_with = "second_deserialize")]
    pub val: Second<Fdim>,
}
impl Default for EndTime {
    fn default() -> Self {
        Self { val: MIN }
    }
}
