use core::fmt;
use std::error::Error;
use std::num::ParseIntError;
use std::str::FromStr;

use super::events::*;

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
    fn from(value: ParseIntError) -> ParseError {
        ParseError::ConversionFailed
    }
}

impl FromStr for NodeAddInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() < 5 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_add" {
            return Err(ParseError::InvalidEvent);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            ip_id: from_hex(split[3])?,
            node_id: from_hex(split[4])?,
        })
    }
}

fn from_hex(input: &str) -> Result<i32, ParseIntError> {
    let without_prefix = input.trim_start_matches("0x");
    i32::from_str_radix(without_prefix, 16)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_hex() {
        println!("{:?}", from_hex("00200002"));
    }

    #[test]
    fn parse_node_add() {
        let input = "node_add 0x00200002 0x0020000D 0x05200002 0x04E00002";
        let node_add_info: NodeAddInfo = input.parse().unwrap();

        println!("{node_add_info:#?}");
    }
}
