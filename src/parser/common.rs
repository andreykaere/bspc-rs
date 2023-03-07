use super::super::events::*;
use super::errors::ParseError;

use std::str::FromStr;

impl FromStr for Stack {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "below" {
            return Ok(Stack::Below);
        }

        if input == "above" {
            return Ok(Stack::Above);
        }

        Err(ParseError::ConversionFailed)
    }
}

impl FromStr for Geometry {
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
