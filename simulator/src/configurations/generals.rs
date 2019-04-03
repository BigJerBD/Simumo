/*! Define general configuration. */

use crate::commons::metrics::second_deserialize;
use crate::commons::metrics::Fdim;
use crate::configurations::debugger::VisualDebugger;
use dim::si::{Second, MIN};

#[derive(Deserialize)]
pub struct GeneralConfigurations {
    #[serde(deserialize_with = "second_deserialize")]
    ///Represent the step ins second between each tick of clock.
    pub clock_dt: Second<Fdim>,
    pub end_time: EndTime,
    pub debugger: VisualDebugger,
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
