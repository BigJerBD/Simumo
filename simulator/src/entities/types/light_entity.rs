use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::dynamic::Position;
use crate::components::types::statics::trafficlight::Light;
use crate::entities::entity_type::Instantiable;
use crate::metrics::identifier_deserialize;
use crate::ressources::eventsmanagement::EventsManager;
use crate::systems::renderer::drawableshape::Circle;
use crate::systems::renderer::drawableshape::DrawableShape;
use specs::World;
use specs::prelude::{Entities, LazyUpdate, Read};
use specs::Builder;

#[derive(Deserialize, Debug)]
pub struct LightEntity {
    #[serde(deserialize_with = "identifier_deserialize")]
    pub id: Identifier,
    pub light: Light,
    #[serde(default)]
    pub position: Position,
    pub observable: String,
}

impl LightEntity {
    fn connect_to_observable(&self, world: &mut World, id_observable: String) {
        let mut events_manager = world.write_resource::<EventsManager>();
        events_manager.connect(id_observable, self.id.0.clone());
    }
}

impl<'a> Instantiable<'a> for LightEntity {
    fn create(&self, world: &mut World) {
        self.connect_to_observable(world, self.observable.clone());
        world
            .create_entity()
            .with(self.id.clone())
            .with(self.light)
            .with(self.position.clone())
            .with(Drawer {
                figure: DrawableShape::Circle(Circle::new(4.0)),
            })
            .build();
    }
    fn spawn(&self, entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>) {
        let entity = entities.create();
        updater.insert(entity, self.id.clone());
        updater.insert(entity, self.light);
        updater.insert(entity, self.position.clone());
        updater.insert(
            entity,
            Drawer {
                figure: DrawableShape::Circle(Circle::new(4.0)),
            },
        );
    }
}
