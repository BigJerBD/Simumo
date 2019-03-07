use crate::entities::entity_prelude::*;
use crate::entities::types::CarEntity;
use crate::entities::types::LightEntity;

pub trait Instantiable<'a> {
    fn create(&self, world: &mut World);
    fn spawn(&self, entities: &Entities<'a>, updater: Read<'a, LazyUpdate>);
}

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum EntityType {
    #[serde(rename = "vehicle")]
    CarEntity(CarEntity),
    #[serde(rename = "trafficlight")]
    LightEntity(LightEntity)
}

/*#[derive(Deserialize)]
enum EntitiesTypes<'a> {
    CarEntity(CarEntity),
    LightEntity(LightEntity)
}*/

/*impl<'a> Spawnable for EntitiesTypes<'a>{
    fn spawn(&self, updater: Read<'a, LazyUpdate>){
        match self {
            CarEntity(car) => car.create(),
            LightEntity(light) => 
        }
    }
}*/