use crate::systems::sys_prelude::*;
use crate::systems::mobility::StandardMobilitySystem;

pub trait MobilitySystemType {}
impl SystemTypeDefinition for MobilitySystemType {

    fn set_dependencies(name:String,dependencies: &mut SystemDependencies) {
        dependencies.mobility = name;
    }

    fn get_dependencies(dependencies:&SystemDependencies) -> Vec<String>{
        vec![dependencies.physic.clone()]
    }



    type SubSystems = ();
}




enum MobilitySystems {
    StandardMobilitySys(StandardMobilitySystem)
}