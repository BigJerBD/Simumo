pub use std::any::Any;

pub use crate::errors::InvalidNameError;

use crate::internal_prelude::*;
use crate::systems::system_dependencies::SystemDependencies;

pub trait SystemTypeDefinition {
    type SubSystems;

    fn set_dependencies(name: String, dependencies: &mut SystemDependencies);
    fn get_dependencies(dependencies: &SystemDependencies) -> Vec<String>;
    fn get_name<T: TypeInfo>() -> String {
        T::typestring()
    }
}
