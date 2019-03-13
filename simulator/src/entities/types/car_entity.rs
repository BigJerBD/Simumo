use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::dynamic::Acceleration;
use crate::components::types::dynamic::Position;
use crate::components::types::dynamic::Speed;
use crate::entities::entity_type::Instantiable;
use crate::metrics::identifier_deserialize;
use crate::systems::renderer::drawableshape::DrawableShape;
use crate::systems::renderer::drawableshape::Rectangle;
use specs::World;
use specs::prelude::{Entities, LazyUpdate, Read};
use specs::Builder;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarEntity {
    #[serde(deserialize_with = "identifier_deserialize")]
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

impl<'a> Instantiable<'a> for CarEntity {
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.id.clone())
            .with(self.position.clone())
            .with(self.speed.clone())
            .with(Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            })
            .build();
    }
    fn spawn(&self, entities: &Entities<'a>, updater: Read<'a, LazyUpdate>) {
        let entity = entities.create();
        updater.insert(entity, self.id.clone());
        updater.insert(entity, self.position.clone());
        updater.insert(entity, self.speed.clone());
        updater.insert(
            entity,
            Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            },
        );
    }
}
