use crate::systems::physic::AccelerationSys;
use crate::systems::sys_prelude::*;

pub trait PhysicSysType {}
impl SystemTypeDefinition for PhysicSysType {

    fn set_dependencies(name:String,dependencies: &mut SystemDependencies) {
        dependencies.recorders.push(name);
    }

    fn get_dependencies(dependencies:&SystemDependencies) -> Vec<String>{
        vec![dependencies.mobility.clone()]
    }

    fn build_subsystem(name: String) -> Result<Box<Any>,InvalidNameError> {
        match name.as_str() {
            AccelerationSys::typename => Ok(AccelerationSys.in_anybox()),
            _ => Err(invalid_system(&name))
        }
    }
}



