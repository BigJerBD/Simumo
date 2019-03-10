use crate::components::types::constant::Identifier;
use crate::components::types::statics::trafficlight::Light;
use crate::entities::entity_type::Instantiable;
use crate::metrics::identifier_deserialize;
use specs::prelude::{Entities, LazyUpdate, Read};
use specs::Builder;
use specs::World;
use crate::components::types::dynamic::Position;
use crate::components::types::constant::Drawer;
use crate::systems::renderer::drawableshape::DrawableShape;
use crate::systems::renderer::drawableshape::Circle;

#[derive(Deserialize, Debug)]
pub struct LightEntity {
    #[serde(deserialize_with = "identifier_deserialize")]
    pub id: Identifier,
    pub light: Light,
    #[serde(default)]
    pub position: Position
}

impl<'a> Instantiable<'a> for LightEntity {
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.id.clone())
            .with(self.light)
            .with(self.position.clone())
            .with(Drawer {
                figure: DrawableShape::Circle(Circle::new(4.0))
            })
            .build();
    }
    fn spawn(&self, entities: &Entities<'a>, updater: Read<'a, LazyUpdate>) {
        let entity = entities.create();
        updater.insert(entity, self.id.clone());
        updater.insert(entity, self.light);
        updater.insert(entity, self.position.clone());
        updater.insert(
            entity,
            Drawer {
                figure: DrawableShape::Circle(Circle::new(4.0))
            }
        );
    }
}
