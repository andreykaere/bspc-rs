use super::super::events::*;
use super::errors::ParseError;
use super::utils::{from_hex, get_event_type, process_event_reply};

use std::str::FromStr;

impl FromStr for NodeAddInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_add", 4)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            ip_id: from_hex(reply[3])?,
            node_id: from_hex(reply[4])?,
        })
    }
}

impl FromStr for NodeRemoveInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_remove", 3)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
        })
    }
}

impl FromStr for NodeSwapInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_swap", 6)?;

        Ok(Self {
            src_monitor_id: from_hex(reply[1])?,
            src_desktop_id: from_hex(reply[2])?,
            src_node_id: from_hex(reply[3])?,
            dst_monitor_id: from_hex(reply[4])?,
            dst_desktop_id: from_hex(reply[5])?,
            dst_node_id: from_hex(reply[6])?,
        })
    }
}

impl FromStr for NodeTransferInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_transfer", 6)?;

        Ok(Self {
            src_monitor_id: from_hex(reply[1])?,
            src_desktop_id: from_hex(reply[2])?,
            src_node_id: from_hex(reply[3])?,
            dst_monitor_id: from_hex(reply[4])?,
            dst_desktop_id: from_hex(reply[5])?,
            dst_node_id: from_hex(reply[6])?,
        })
    }
}

impl FromStr for NodeFocusInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_focus", 3)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
        })
    }
}

impl FromStr for NodeActivateInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_activate", 3)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
        })
    }
}

impl FromStr for NodePreselInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_presel", 4)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
            presel: reply[4].parse()?,
        })
    }
}

impl FromStr for NodeStackInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_stack", 3)?;

        Ok(Self {
            node_id_1: from_hex(reply[1])?,
            stack: reply[2].parse()?,
            node_id_2: from_hex(reply[3])?,
        })
    }
}

impl FromStr for NodeLayerInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_layer", 4)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
            layer: reply[4].parse()?,
        })
    }
}

impl FromStr for NodeFlagInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_flag", 5)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
            flag: reply[4].parse()?,
            switch: reply[5].parse()?,
        })
    }
}

impl FromStr for NodeStateInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_state", 5)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
            state: reply[4].parse()?,
            switch: reply[5].parse()?,
        })
    }
}

impl FromStr for NodeGeometryInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let reply = process_event_reply(input, "node_geometry", 4)?;

        Ok(Self {
            monitor_id: from_hex(reply[1])?,
            desktop_id: from_hex(reply[2])?,
            node_id: from_hex(reply[3])?,
            node_geometry: reply[4].parse()?,
        })
    }
}

impl FromStr for NodeEvent {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let event_type = get_event_type(input)?;

        match event_type {
            "node_add" => Ok(NodeEvent::NodeAdd(input.parse()?)),
            "node_remove" => Ok(NodeEvent::NodeRemove(input.parse()?)),
            "node_swap" => Ok(NodeEvent::NodeSwap(input.parse()?)),
            "node_transfer" => Ok(NodeEvent::NodeTransfer(input.parse()?)),
            "node_focus" => Ok(NodeEvent::NodeFocus(input.parse()?)),
            "node_activate" => Ok(NodeEvent::NodeActivate(input.parse()?)),
            "node_presel" => Ok(NodeEvent::NodePresel(input.parse()?)),
            "node_stack" => Ok(NodeEvent::NodeStack(input.parse()?)),
            "node_geometry" => Ok(NodeEvent::NodeGeometry(input.parse()?)),
            "node_state" => Ok(NodeEvent::NodeState(input.parse()?)),
            "node_flag" => Ok(NodeEvent::NodeFlag(input.parse()?)),
            "node_layer" => Ok(NodeEvent::NodeLayer(input.parse()?)),
            _ => Err(ParseError::ConversionFailed),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_node_add() {
        let input = "node_add 0x00200002 0x0020000D 0x05200002 0x04E00002";
        let node_add_info: NodeAddInfo = input.parse().unwrap();

        println!("{node_add_info:#?}");
    }
    #[test]
    fn parse_node_focus() {
        let input = "node_focus 0x00200002 0x00200007 0x07800002";
        let node_focus_info: NodeFocusInfo = input.parse().unwrap();

        println!("{node_focus_info:#?}");
    }
    #[test]
    fn parse_node_stack() {
        let input = "node_stack 0x07800002 below 0x04200003";
        let node_stack_info: NodeStackInfo = input.parse().unwrap();

        println!("{node_stack_info:#?}");
    }
    #[test]
    fn parse_node_geometry() {
        let input =
            "node_geometry 0x00200002 0x00200007 0x07800002 681x365+0+403\n";
        let node_geometry_info: NodeGeometryInfo = input.parse().unwrap();

        println!("{node_geometry_info:#?}");
    }

    #[test]
    fn parse_node_event() {
        let tests = [
            "node_state 0x00200002 0x00200007 0x04400002 tiled on",
            "node_add 0x00200002 0x00200007 0x04400002 0x08600002",
            "node_geometry 0x00200002 0x00200007 0x04400002 677x361+0+35",
            "node_stack 0x04400002 below 0x04200003",
            "node_state 0x00200002 0x00200007 0x04400002 fullscreen off",
        ];

        for test in tests {
            let data: NodeEvent = test.parse().unwrap();

            println!("{data:#?}")
        }
    }
}
