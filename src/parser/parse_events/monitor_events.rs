use super::*;

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
            "monitor_add" => Ok(MonitorEvent::MonitorAdd(input.parse()?)),
            "monitor_rename" => Ok(MonitorEvent::MonitorRename(input.parse()?)),
            "monitor_remove" => Ok(MonitorEvent::MonitorRemove(input.parse()?)),
            "monitor_swap" => Ok(MonitorEvent::MonitorSwap(input.parse()?)),
            "monitor_focus" => Ok(MonitorEvent::MonitorFocus(input.parse()?)),
            "monitor_geometry" => {
                Ok(MonitorEvent::MonitorGeometry(input.parse()?))
            }

            _ => Err(ParseError::ConversionFailed),
        }
    }
}
