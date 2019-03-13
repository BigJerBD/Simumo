use dim::si::{Second, MIN};

use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

#[derive(Deserialize)]
pub struct GeneralConfigurations {
    pub end_time: EndTime,
    pub seed: String,
}

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
