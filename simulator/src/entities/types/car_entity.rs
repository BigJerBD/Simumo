use crate::commons::CartesianCoord;
use crate::commons::Percentage;
use crate::commons::PolarCoord;
use crate::components::types::constant::CarType;
use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::entities::entity_type::Instantiable;
use crate::ressources::lane_graph::NodeId;
use crate::systems::renderer::drawableshape::DrawableShape;
use crate::systems::renderer::drawableshape::Rectangle;
use dim::si::MPS;
use specs::prelude::{Builder, Entities, LazyUpdate, Read, World};
use specs::EntityBuilder;

#[derive(Serialize, Deserialize, Debug)]
pub struct CarEntity {
    pub id: String,
    //mass : Mass,
    //length : Length,
    //angle: Angle,
    #[serde(default)]
    pub position: ((NodeId, NodeId), f64),
    #[serde(default)]
    pub destination: (f64, f64),
    #[serde(default)]
    pub speed: f64,
    #[serde(default)]
    pub acceleration: f64,
    //energy_control: EnergyControl,
    //agent_type:
}

impl<'a> Instantiable<'a> for CarEntity {
    // NOTE :: a create car is converted to the cartesian referential
    // but a spawned one is already on the cartesian referential
    fn create(&self, world: &mut World, is_rendering_on: bool) {
        let mut entity_builder: EntityBuilder = world
            .create_entity()
            .with(Identifier(self.id.clone()))
            .with(Position {
                val: (self.position.0, Percentage::new_clamp(self.position.1)),
            })
            /*.with(Destination {
                val: polarfloat_to_cartesian(self.destination.1, self.destination.0),
            })*/
            .with(CarType)
            .with(Speed {
                speed: self.speed * MPS,
            });
        if is_rendering_on {
            entity_builder = entity_builder.with(Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            });
        }
        entity_builder.build();
    }

    fn spawn(
        &self,
        entities: &Entities<'a>,
        updater: &Read<'a, LazyUpdate>,
        is_rendering_on: bool,
    ) {
        let entity = entities.create();
        updater.insert(entity, Identifier(self.id.clone()));
        updater.insert(
            entity,
            Position {
                val: (self.position.0, Percentage::new_clamp(self.position.1)),
            },
        );
        /*updater.insert(
            entity,
            Destination {
                val: CartesianCoord::from_float(self.destination.0, self.destination.1),
            },
        );*/
        updater.insert(entity, CarType);
        updater.insert(
            entity,
            Speed {
                speed: self.speed * MPS,
            },
        );
        if is_rendering_on {
            updater.insert(
                entity,
                Drawer {
                    figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
                },
            );
        }
    }
}

/// for convenience
fn polarfloat_to_cartesian(lat: f64, lon: f64) -> CartesianCoord {
    let polar = PolarCoord::from_float(lat, lon);
    CartesianCoord::from_polar(&polar)
}
