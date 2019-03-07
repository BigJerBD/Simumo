use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

use dim::si::{Second, MIN};

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
