use super::super::events::*;
use super::errors::ParseError;
use super::utils::{from_hex, get_event_type, process_event_reply};

use std::str::FromStr;

impl FromStr for DesktopAddInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_add", 3)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            desktop_name: reply[3].to_string(),
        })
    }
}

impl FromStr for DesktopRenameInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_rename", 4)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            old_name: reply[3].to_string(),
            new_name: reply[4].to_string(),
        })
    }
}

impl FromStr for DesktopRemoveInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_remove", 2)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
        })
    }
}

impl FromStr for DesktopSwapInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_swap", 4)?;

        Ok(Self {
            src_monitor_id: from_hex(reply[1])?,
            src_desktop_id: from_hex(reply[2])?,
            dst_monitor_id: from_hex(reply[3])?,
            dst_desktop_id: from_hex(reply[4])?,
        })
    }
}

impl FromStr for DesktopTransferInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_transfer", 3)?;

        Ok(Self {
            src_monitor_id: from_hex(reply[1])?,
            src_desktop_id: from_hex(reply[2])?,
            dst_monitor_id: from_hex(reply[3])?,
        })
    }
}

impl FromStr for DesktopFocusInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_focus", 2)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
        })
    }
}

impl FromStr for DesktopActivateInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_activate", 2)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
        })
    }
}

impl FromStr for DesktopLayoutInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "desktop_layout", 2)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            layout: reply[3].parse()?,
        })
    }
}

impl FromStr for DesktopEvent {
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