use std::error::Error;
use std::fmt::{self, Display, Formatter};

type Fdef = f64;

#[derive(Debug)]
pub struct OutOfRange;

impl Error for OutOfRange {}

impl Display for OutOfRange {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Value out of range")
    }
}

pub struct Percentage(f64);

impl Percentage {
    pub fn new(value: Fdef) -> Result<Percentage, OutOfRange> {
        const LOWER: Fdef = 0.0;
        const UPPER: Fdef = 100.0;

        if value >= LOWER && value <= UPPER {
            Ok(Percentage(value))
        } else {
            Err(OutOfRange)
        }
    }

    pub fn value(&self) -> Fdef {
        self.0
    }
}
