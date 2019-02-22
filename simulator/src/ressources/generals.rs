use dim::si::{Second, MIN};

use crate::internal_prelude::*;
use crate::metrics::Fdim;
use crate::metrics::second_deserialize;

#[simuresource]
pub struct LogDirectory {
    pub val: String,
}

#[derive(Deserialize)]
pub struct EndTime {
    #[serde(deserialize_with="second_deserialize")]
    pub val: Second<Fdim>,
}
impl Default for EndTime {
    fn default() -> Self {
        Self { val: MIN }
    }
}
