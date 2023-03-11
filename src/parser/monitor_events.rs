use super::super::events::*;
use super::errors::ParseError;
use super::utils::{from_hex, get_event_type, process_event_reply};

use std::str::FromStr;

impl FromStr for MonitorAddInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "monitor_add", 3)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            monitor_name: reply[2].to_string(),
            monitor_geometry: reply[3].parse()?,
        })
    }
}

impl FromStr for MonitorRenameInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "monitor_rename", 3)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            old_name: reply[2].to_string(),
            new_name: reply[3].to_string(),
        })
    }
}

impl FromStr for MonitorRemoveInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "monitor_remove", 1)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
        })
    }
}

impl FromStr for MonitorSwapInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "monitor_swap", 2)?;

        Ok(Self {
            src_monitor_id: from_hex(reply[1])?,
            dst_monitor_id: from_hex(reply[2])?,
        })
    }
}

impl FromStr for MonitorFocusInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "monitor_focus", 1)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
        })
    }
}

impl FromStr for MonitorGeometryInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "monitor_geometry", 2)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            monitor_geometry: reply[2].parse()?,
        })
    }
}

impl FromStr for MonitorEvent {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let event_type = get_event_type(input)?;

        match event_type {
            // "node_add" => Ok(NodeEvent::NodeAdd(input.parse()?)),
            // "node_remove" => Ok(NodeEvent::NodeRemove(input.parse()?)),
            // "node_swap" => Ok(NodeEvent::NodeSwap(input.parse()?)),
            // "node_transfer" => Ok(NodeEvent::NodeTransfer(input.parse()?)),
            // "node_focus" => Ok(NodeEvent::NodeFocus(input.parse()?)),
            // "node_activate" => Ok(NodeEvent::NodeActivate(input.parse()?)),
            // "node_presel" => Ok(NodeEvent::NodePresel(input.parse()?)),
            // "node_stack" => Ok(NodeEvent::NodeStack(input.parse()?)),
            // "node_geometry" => Ok(NodeEvent::NodeGeometry(input.parse()?)),
            // "node_state" => Ok(NodeEvent::NodeState(input.parse()?)),
            // "node_flag" => Ok(NodeEvent::NodeFlag(input.parse()?)),
            // "node_layer" => Ok(NodeEvent::NodeLayer(input.parse()?)),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}
