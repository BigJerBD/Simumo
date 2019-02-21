use crate::systems::mobility::StandardMobilitySystem;
use crate::systems::sys_prelude::*;

pub trait MobilitySystemType {}
impl SystemTypeDefinition for MobilitySystemType {
    fn set_dependencies(name: String, dependencies: &mut SystemDependencies) {
        dependencies.mobility = name;
    }

    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String> {
        vec![dependencies.physic.clone()]
    }

    type SubSystems = ();
}

enum MobilitySystems {
    StandardMobilitySys(StandardMobilitySystem),
}
