use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::dynamic::Acceleration;
use crate::components::types::dynamic::Position;
use crate::components::types::dynamic::Speed;
use crate::entities::entity_type::Instantiable;
use crate::metrics::identifier_deserialize;
use crate::systems::renderer::drawableshape::DrawableShape;
use crate::systems::renderer::drawableshape::Rectangle;
use specs::{Builder, World};
use specs::prelude::{Entities, LazyUpdate, Read};
use dim::si::{M, MPS, MPS2};

#[derive(Serialize, Deserialize, Debug)]
pub struct CarEntity {
    pub id: String,
    //mass : Mass,
    //length : Length,
    //angle: Angle,
    #[serde(default)]
    pub position: (f64, f64),
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub acceleration: f64,
    //energy_control: EnergyControl,
    //agent_type:
}

impl<'a> Instantiable<'a> for CarEntity {
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(Identifier(self.id.clone()))
            .with(Position {
                x: self.position.0 * M,
                y: self.position.1 * M,
            })
            .with(Speed {
                val: self.speed * MPS
            })
            .with(Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            })
            .build();
    }
    fn spawn(&self, entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>) {
        let entity = entities.create();
        updater.insert(entity, Identifier(self.id.clone()));
        updater.insert(
            entity,
            Position {
                x: self.position.0 * M,
                y: self.position.1 * M,
            }
        );
        updater.insert(
            entity,
            Speed {
                val: self.speed * MPS
            });
        updater.insert(
            entity,
            Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            },
        );
    }
}
