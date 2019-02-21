use crate::systems::physic::AccelerationSystem;
use crate::systems::sys_prelude::*;

pub trait PhysicSystemType {}
impl SystemTypeDefinition for PhysicSystemType {
    type SubSystems = PhysicSystems;

    fn set_dependencies(name:String,dependencies: &mut SystemDependencies) {
        dependencies.recorders.push(name);
    }

    fn get_dependencies(dependencies:&SystemDependencies) -> Vec<String>{
        vec![dependencies.mobility.clone()]
    }
}


#[derive(Deserialize)]
pub enum PhysicSystems {
    AccelerationSystem(AccelerationSystem)
}
