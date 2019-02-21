use dim::si::{MIN, Second};

use crate::metrics::Fdim;

#[derive(Default)]
pub struct LogDirectory {
    pub val: String,
}

pub struct EndTime {
    pub val: Second<Fdim>,
}

impl Default for EndTime {
    fn default() -> Self {
        Self{ val: MIN }
    }
}
