use std::collections::HashMap;

use crate::errors::InvalidNameError;

pub trait SystemDefinition {
    //todo :: should return a result
    fn initialize(&mut self,
                  _config: HashMap<String, FieldValue>,
                  _general_config: HashMap<String, String>) -> Result<(), InvalidNameError>
    { Ok(()) }
}


pub enum FieldValue {
    StringVal(String),
    ArrayVal(Vec<FieldValue>),
}

/// Util function to create an Invalid system name error
pub fn invalid_field(name: &str) -> InvalidNameError {
    InvalidNameError::new("Field", name)
}
