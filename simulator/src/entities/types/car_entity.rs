use crate::components::types::constant::Drawer;
use crate::components::types::constant::Identifier;
use crate::components::types::dynamic::Speed;
use crate::components::Position;
use crate::entities::entity_type::Instantiable;
use crate::systems::renderer::drawableshape::DrawableShape;
use crate::systems::renderer::drawableshape::Rectangle;

use crate::commons::CartesianCoord;
use crate::commons::PolarCoord;
use crate::components::types::constant::CarType;
use dim::si::MPS;
use specs::prelude::{Entities, LazyUpdate, Read};
use specs::Builder;
use specs::World;

///todo :: it is possible to provide a single
/// creation statement by putting all Component in a single iterator (or vector)

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
    // NOTE :: a create car is converted to the cartesian referential
    //         BUT a spawned one is already on the cartesian referential
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(Identifier(self.id.clone()))
            .with(Position {
                val: polarfloat_to_cartesian(self.position.1, self.position.0),
            })
            .with(CarType)
            .with(Speed {
                val: self.speed * MPS,
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
                val: CartesianCoord::from_float(self.position.0, self.position.1),
            },
        );
        updater.insert(entity, CarType);
        updater.insert(
            entity,
            Speed {
                val: self.speed * MPS,
            },
        );
        updater.insert(
            entity,
            Drawer {
                figure: DrawableShape::Rectangle(Rectangle::new(3.0, 3.0)),
            },
        );
    }
}

/// for convenience
fn polarfloat_to_cartesian(lat: f64, lon: f64) -> CartesianCoord {
    let polar = PolarCoord::from_float(lat, lon);
    CartesianCoord::from_polar(&polar)
}
