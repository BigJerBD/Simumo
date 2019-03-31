use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct OutOfRange;

impl Error for OutOfRange {}

impl Display for OutOfRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Value out of range")
    }
}

pub struct Percentage(f32);

impl Percentage {
    pub fn new(value: f32) -> Result<Percentage, OutOfRange> {
        const LOWER: f32 = 0.0;
        const UPPER: f32 = 100.0;

        if value >= LOWER && value <= UPPER {
            Ok(Percentage(value))
        } else {
            Err(OutOfRange)
        }
    }

    pub fn value(&self) -> f32 {
        self.0
    }
}
