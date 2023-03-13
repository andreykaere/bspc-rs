use super::errors::ParseError;
use super::utils::{from_hex_to_id, get_event_type, process_event_reply};
use crate::events::*;

use std::str::FromStr;

pub mod desktop_events;
pub mod monitor_events;
pub mod node_events;

impl FromStr for Event {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = input.parse::<NodeEvent>() {
            return Ok(Event::NodeEvent(x));
        }

        if let Ok(x) = input.parse::<DesktopEvent>() {
            return Ok(Event::DesktopEvent(x));
        }

        if let Ok(x) = input.parse::<MonitorEvent>() {
            return Ok(Event::MonitorEvent(x));
        }

        if let Ok(x) = input.parse::<ReportInfo>() {
            return Ok(Event::Report(x));
        }

        if let Ok(x) = input.parse::<PointerActionInfo>() {
            return Ok(Event::PointerAction(x));
        }

        Err(ParseError::ConversionFailed)
    }
}

impl FromStr for PointerActionInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "pointer_action", 5)?;

        Ok(Self {
            monitor_id: from_hex_to_id(reply[1])?,
            desktop_id: from_hex_to_id(reply[2])?,
            node_id: from_hex_to_id(reply[3])?,
            action: reply[4].parse()?,
            action_state: reply[5].parse()?,
        })
    }
}

impl FromStr for ReportInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!();
    }
}
