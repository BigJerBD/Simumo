use crate::components::type_prelude::*;

#[derive(Clone, Component, Serialize, Deserialize, Debug)]
#[storage(VecStorage)]
#[serde(tag = "type")]
pub enum SpawnerType {
    Frequency(FrequencySpawner)
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct FrequencySpawner {
    pub period: f64
}