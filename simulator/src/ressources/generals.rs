use crate::internal_prelude::*;
use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

use dim::si::{Second, MIN};
use std::cell::RefCell;
use std::sync::Mutex;

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
