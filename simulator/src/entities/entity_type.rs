use crate::entities::entity_prelude::*;
use crate::entities::types::CarEntity;
use crate::entities::types::LightEntity;

pub trait Instantiable<'a> {
    fn create(&self, world: &mut World);
    fn spawn(&self, entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>);
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EntityType {
    #[serde(rename = "vehicle")]
    CarEntity(CarEntity),
    #[serde(rename = "trafficlight")]
    LightEntity(LightEntity),
}

impl<'a> Instantiable<'a> for EntityType {
    fn create(&self, world: &mut World) {
        match self {
            EntityType::CarEntity(car) => car.create(world),
            EntityType::LightEntity(light) => light.create(world),
        }
    }
    fn spawn(&self, entities: &Entities<'a>, updater: &Read<'a, LazyUpdate>) {
        match self {
            EntityType::CarEntity(car) => car.spawn(entities, updater),
            EntityType::LightEntity(light) => light.spawn(entities, updater),
        }
    }
}
