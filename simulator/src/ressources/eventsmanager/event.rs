use crate::{TrafficLightColor};

#[derive(Clone, Debug)]
pub enum Event {
    TrafficLightColorChange(TrafficLightColor)
}