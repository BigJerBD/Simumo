use crate::systems::sys_prelude::*;
use crate::systems::mobility::StandardMobilitySys;

pub trait MobilitySysType {}
impl SystemTypeDefinition for MobilitySysType {

    fn set_dependencies(name:String,dependencies: &mut SystemDependencies) {
        dependencies.mobility = name;
    }

    fn get_dependencies(dependencies:&SystemDependencies) -> Vec<String>{
        vec![dependencies.physic.clone()]
    }

    fn build_subsystem(name: String) -> Result<Box<Any>,InvalidNameError> {
        match name.as_str() {
            StandardMobilitySys::typename => Ok(StandardMobilitySys.in_anybox()),
            _ => Err(invalid_system(&name))
        }
    }
}


