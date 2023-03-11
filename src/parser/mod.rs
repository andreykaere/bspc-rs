use super::errors::ParseError;
use crate::properties::*;

use std::str::FromStr;

pub mod errors;
pub mod parse_events;
mod utils;

impl FromStr for Presel {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        match split[0] {
            "dir" => Ok(Self::Dir(split[1].parse()?)),
            "ratio" => Ok(Self::Ratio(split[1].parse()?)),
            "cancel" => Ok(Self::Cancel),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Rectangle {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(&['+', 'x'][..]).collect();

        if split.len() < 4 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            width: split[0].parse()?,
            height: split[1].parse()?,
            x: split[2].parse()?,
            y: split[3].parse()?,
        })
    }
}
