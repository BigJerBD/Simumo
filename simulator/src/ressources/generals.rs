use dim::si::{Second, MIN};

use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

#[derive(Clone, Deserialize)]
pub struct EndTime {
    #[serde(deserialize_with = "second_deserialize")]
    pub val: Second<Fdim>,
}

impl Default for EndTime {
    fn default() -> Self {
        Self { val: MIN }
    }
}

pub struct MapBbox {
    pub x1: f64,
    pub x2: f64,
    pub y1: f64,
    pub y2: f64,
}
