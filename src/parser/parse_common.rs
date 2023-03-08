use super::super::common::*;
use super::errors::ParseError;

use std::str::FromStr;

impl FromStr for Layout {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "monocle" => Ok(Self::Monocle),
            "tiled" => Ok(Self::Tiled),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Action {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "move" => Ok(Self::Move),
            "resize_corner" => Ok(Self::ResizeCorner),
            "resize_side" => Ok(Self::ResizeSide),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for ActionState {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "begin" => Ok(Self::Begin),
            "end" => Ok(Self::End),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Layer {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "below" => Ok(Self::Below),
            "above" => Ok(Self::Above),
            "normal" => Ok(Self::Normal),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Stack {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "below" => Ok(Self::Below),
            "above" => Ok(Self::Above),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Switch {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "on" => Ok(Self::On),
            "off" => Ok(Self::Off),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Flag {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "hidden" => Ok(Self::Hidden),
            "sticky" => Ok(Self::Sticky),
            "private" => Ok(Self::Private),
            "locked" => Ok(Self::Locked),
            "marked" => Ok(Self::Marked),
            "urgent" => Ok(Self::Urgent),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for Dir {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "south" => Ok(Self::South),
            "north" => Ok(Self::North),
            "west" => Ok(Self::West),
            "east" => Ok(Self::East),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

impl FromStr for State {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "tiled" => Ok(Self::Tiled),
            "pseudo_tiled" => Ok(Self::PseudoTiled),
            "floating" => Ok(Self::Floating),
            "fullscreen" => Ok(Self::Fullscreen),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

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
