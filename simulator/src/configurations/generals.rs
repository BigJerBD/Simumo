/*! Define general configuration. */

use dim::si::{Second, MIN};
use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

#[derive(Deserialize)]
pub struct GeneralConfigurations {
    #[serde(deserialize_with = "second_deserialize")]
    ///Represent the step ins second between each tick of clock.
    pub clock_dt: Second<Fdim>,
    pub end_time: EndTime,
    ///A unique seed will be generate if there is no seed in the configuration file.
    pub seed: String,
}

///Represent the ending time of the simulator.
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
