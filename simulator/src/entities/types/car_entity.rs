use crate::components::types::dynamic::Position;
use crate::components::types::constant::Length;
use crate::components::types::dynamic::Speed;
use crate::components::constant::Mass;
use crate::components::types::dynamic::Angle;
use crate::components::types::dynamic::Acceleration;
use crate::components::types::controls::EnergyControl;
use crate::components::types::constant::CarType;
use crate::components::types::constant::Identifier;
use crate::components::types::constant::Rectangle;
use crate::entities::entity_type::Creatable;
use specs::World;
use specs::Builder;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarEntity {
    pub id: Identifier,
    //mass : Mass,
    //length : Length,
    //angle: Angle,
    #[serde(default)]
    pub position: Position,
    #[serde(default)]
    pub speed: Speed,
    #[serde(default)]
    pub acceleration: Acceleration,
    //energy_control: EnergyControl,
    //agent_type:
}

impl Creatable for CarEntity {
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.id.clone())
            .with(self.position.clone())
            .with(self.speed.clone())
            .with(CarType)
            .with(Rectangle { width: 5.0, height: 5.0 })
            .build();
    }
}