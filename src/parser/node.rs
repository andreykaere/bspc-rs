use super::super::events::*;
use super::errors::ParseError;
use super::utils::from_hex;

use std::str::FromStr;

impl FromStr for NodeAddInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_add" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 5 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            ip_id: from_hex(split[3])?,
            node_id: from_hex(split[4])?,
        })
    }
}

impl FromStr for NodeRemoveInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_remove" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 4 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
        })
    }
}

impl FromStr for NodeSwapInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_swap" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 7 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            src_monitor_id: from_hex(split[1])?,
            src_desktop_id: from_hex(split[2])?,
            src_node_id: from_hex(split[3])?,
            dst_monitor_id: from_hex(split[4])?,
            dst_desktop_id: from_hex(split[5])?,
            dst_node_id: from_hex(split[6])?,
        })
    }
}

impl FromStr for NodeTransferInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_transfer" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 7 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            src_monitor_id: from_hex(split[1])?,
            src_desktop_id: from_hex(split[2])?,
            src_node_id: from_hex(split[3])?,
            dst_monitor_id: from_hex(split[4])?,
            dst_desktop_id: from_hex(split[5])?,
            dst_node_id: from_hex(split[6])?,
        })
    }
}

impl FromStr for NodeFocusInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_focus" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 4 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
        })
    }
}

impl FromStr for NodeActivateInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_activate" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 4 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
        })
    }
}

impl FromStr for NodePreselInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_presel" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 5 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
            presel: split[4].parse()?,
        })
    }
}

impl FromStr for NodeStackInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_stack" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 4 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            node_id_1: from_hex(split[1])?,
            stack: split[2].parse()?,
            node_id_2: from_hex(split[3])?,
        })
    }
}

impl FromStr for NodeLayerInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_layer" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 5 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
            layer: split[4].parse()?,
        })
    }
}

impl FromStr for NodeFlagInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_flag" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 6 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
            flag: split[4].parse()?,
            switch: split[5].parse()?,
        })
    }
}

impl FromStr for NodeStateInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_state" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 6 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
            state: split[4].parse()?,
            switch: split[5].parse()?,
        })
    }
}

impl FromStr for NodeGeometryInfo {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        if split.len() == 0 {
            return Err(ParseError::InsufficientData);
        }

        if split[0] != "node_geometry" {
            return Err(ParseError::InvalidEvent);
        }

        if split.len() < 5 {
            return Err(ParseError::InsufficientData);
        }

        Ok(Self {
            monitor_id: from_hex(split[1])?,
            desktop_id: from_hex(split[2])?,
            node_id: from_hex(split[3])?,
            node_geometry: split[4].parse()?,
        })
    }
}

impl FromStr for NodeEvent {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<_> = input.split(' ').collect();

        match split[0] {
            "node_add" => Ok(NodeEvent::NodeAdd(input.parse()?)),
            "node_remove" => Ok(NodeEvent::NodeRemove(input.parse()?)),
            "node_swap" => Ok(NodeEvent::NodeSwap(input.parse()?)),
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
}