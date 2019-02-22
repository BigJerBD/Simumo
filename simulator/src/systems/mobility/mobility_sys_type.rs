use crate::systems::mobility::StandardMobilitySystem;
use crate::systems::sys_prelude::*;

pub trait MobilitySystemType {}
impl SystemTypeDefinition for MobilitySystemType {
    type SubSystems = MobilitySystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.mobility = name;
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.physic.clone()]
    }


}

#[derive(Deserialize)]
pub enum MobilitySystems {
    StandardMobilitySys(StandardMobilitySystem),
}
