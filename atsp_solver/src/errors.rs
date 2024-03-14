use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    OutOfRange,
    DimensionMismatch,
    LengthMismatch,
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::OutOfRange => write!(f, "Not all solution elements are in the valid range"),
            MyError::DimensionMismatch => write!(f, "Solution dimension does not match"),
            MyError::LengthMismatch => write!(f, "Solution length does not match dimension"),
        }
    }
}

impl Error for MyError {}