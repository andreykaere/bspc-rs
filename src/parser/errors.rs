use core::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParseError {
    InsufficientData,
    InvalidEvent,
    ConversionFailed,
}

impl Error for ParseError {}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::InsufficientData => {
                write!(f, "Insufficient data was provided")
            }
            ParseError::InvalidEvent => {
                write!(f, "Event of different type was provided")
            }
            ParseError::ConversionFailed => {
                write!(f, "Conversion failed")
            }
        }
    }
}

impl std::convert::From<ParseIntError> for ParseError {
    fn from(_: ParseIntError) -> ParseError {
        ParseError::ConversionFailed
    }
}
