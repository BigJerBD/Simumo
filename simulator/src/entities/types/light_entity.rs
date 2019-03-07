use crate::components::types::dynamic::Position;
use crate::components::types::constant::Identifier;
use crate::components::types::statics::trafficlight::Light;
use specs::World;
use specs::Builder;

#[derive(Deserialize, Debug)]
pub struct LightEntity{
    pub id: Identifier,
    pub light: Light
}

impl LightEntity {
    fn create(&self, world: &mut World) {
        world
            .create_entity()
            .with(self.id.clone())
            .with(self.light.clone())
            .build();
    }
}