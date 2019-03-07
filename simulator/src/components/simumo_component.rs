use crate::components::constant::Identifier;
use crate::components::constant::Mass;
use crate::components::dynamic::Acceleration;
use crate::components::dynamic::Angle;
use crate::components::dynamic::Position;
use crate::components::types::agents::AcceleratingAgent;
use crate::components::types::constant::CarType;
use crate::components::types::controls::EnergyControl;
use crate::components::types::dynamic::Speed;

pub trait SimumoSerialize {}

#[derive(Deserialize)]
enum ComponentTypes {
    //agents
    AcceleratingAgent(AcceleratingAgent),
    //constants
    Mass(Mass),
    Identifier(Identifier),
    CarType(CarType),
    BikeType(CarType),
    //controls
    EnergyControl(EnergyControl),
    //dynamics
    Position(Position),
    Angle(Angle),
    Speed(Speed),
    Acceleration(Acceleration),
}

///used by simumo derive to write custom log
/// todo :: it should be placed in simumo derive
///  it is simpler for quick fix to put it there but its an abomination of design
#[derive(Serialize)]
pub struct LogDataEntry<T> {
    #[serde(rename="type")]
    typename: String,
    resolution: Option<String>,
    value: T,
}

impl<T> LogDataEntry<T> {
    pub fn new(typename: String, resolution: Option<String>, value: T) -> Self {
        Self {
            typename,
            resolution,
            value,
        }

    }
}