use dim::si::{Second, MIN};

use crate::metrics::second_deserialize;
use crate::metrics::Fdim;

#[derive(Deserialize)]
pub struct GeneralConfigurations {
    #[serde(deserialize_with = "second_deserialize")]
    pub clock_dt: Second<Fdim>,
    pub end_time: EndTime,
    pub debugger: VisualDebugger,
    pub seed: String,
}

#[derive(Clone, Deserialize)]
pub struct VisualDebugger {
    #[serde(rename = "use")]
    pub on: bool,
    pub width: f64,
    pub height: f64
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
