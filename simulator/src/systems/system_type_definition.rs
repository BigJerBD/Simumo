pub use std::any::Any;

pub use crate::errors::InvalidNameError;

use crate::internal_prelude::*;
use crate::systems::system_dependencies::SystemDependencies;
use std::collections::HashMap;


/// Trait that allow to specify a family of System
/// With a
///
pub trait SystemTypeDefinition {
    fn set_dependencies(name:String, dependencies: &mut SystemDependencies);
    fn get_dependencies(dependencies : & SystemDependencies) -> Vec<String>;

    //using template for this function is unclean
    // because it asssumes that the System  will have TypeInfo
    fn get_name<T:TypeInfo>() -> String{
        T::typestring()
    }

    fn build_subsystem(name: String) -> Result<Box<Any>,InvalidNameError> ;

    fn apply_setting(_setting:HashMap<String,String>){}

}


/// Util function to create an Invalid system name error
pub fn invalid_system(name: &str) -> InvalidNameError {
    InvalidNameError::new("System", name)
}

/// Trait to convert any struct to a unknown movable data
pub trait AnyBoxable {
    fn in_anybox(self) -> Box<Any>;
}
impl<T: 'static> AnyBoxable for T {
    fn in_anybox(self) -> Box<Any> {
        Box::from(self) as Box<Any>
    }
}


