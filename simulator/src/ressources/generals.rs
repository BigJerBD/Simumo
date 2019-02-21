use dim::si::{Second, MIN};

use crate::internal_prelude::*;
use crate::metrics::Fdim;

#[simuresource]
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
