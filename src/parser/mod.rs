use super::events::*;

use std::str::FromStr;

pub mod desktop_events;
pub mod errors;
pub mod monitor_events;
pub mod node_events;
pub mod parse_common;
mod utils;

use errors::ParseError;
use utils::{from_hex, get_event_type, process_event_reply};

impl FromStr for Event {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if let Ok(x) = input.parse::<NodeEvent>() {
            return Ok(Event::Node(x));
        }

        if let Ok(x) = input.parse::<DesktopEvent>() {
            return Ok(Event::Desktop(x));
        }

        if let Ok(x) = input.parse::<MonitorEvent>() {
            return Ok(Event::Monitor(x));
        }

        // if let Ok(x) = input.parse::<ReportInfo>() {
        //     return Ok(Event::Report(x));
        // }

        // if let Ok(x) = input.parse::<PointerActionInfo>() {
        //     return Ok(Event::PointerAction(x));
        // }

        Err(ParseError::ConversionFailed)
    }
}
