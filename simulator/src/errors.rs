use std::error;
use std::fmt;


//enum SimumoError {
//    InvalidNameError()
//}


#[derive(Debug,Clone)]
pub struct InvalidNameError(pub String, pub String);
impl InvalidNameError {
    pub fn new(ttype : &str, value: &str) -> Self {
        Self {
            0: ttype.to_string(),
            1: value.to_string()
        }
    }
}

impl InvalidNameError {}
impl fmt::Display for InvalidNameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "type : {}, value {}",self.0,self.1)
    }
}
impl error::Error for InvalidNameError {
    fn description(&self) -> &str { "Invalid name"}
    fn cause(&self) -> Option<&error::Error> { None }
}
