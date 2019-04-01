/*! Define general configuration. */

use crate::commons::metrics::second_deserialize;
use crate::commons::metrics::Fdim;
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

///Represent the ending time of the simulator.
#[derive(Clone, Deserialize)]
pub struct VisualDebugger {
    #[serde(rename = "use")]
    pub on: bool,
    pub width: f64,
    pub height: f64,
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
