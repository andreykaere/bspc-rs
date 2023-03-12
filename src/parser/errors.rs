use core::fmt;
use std::error::Error;
use std::num::{ParseFloatError, ParseIntError};
use std::str::ParseBoolError;

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
    fn from(_error: ParseIntError) -> ParseError {
        ParseError::ConversionFailed
    }
}

impl std::convert::From<ParseBoolError> for ParseError {
    fn from(_error: ParseBoolError) -> ParseError {
        ParseError::ConversionFailed
    }
}

impl std::convert::From<ParseFloatError> for ParseError {
    fn from(_error: ParseFloatError) -> ParseError {
        ParseError::ConversionFailed
    }
}

impl std::convert::From<serde_json::Error> for ParseError {
    fn from(_error: serde_json::Error) -> ParseError {
        ParseError::ConversionFailed
    }
}

impl std::convert::From<strum::ParseError> for ParseError {
    fn from(_error: strum::ParseError) -> ParseError {
        ParseError::ConversionFailed
    }
}
