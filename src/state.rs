//! This module is dedicated for the state dump of bspwm. It can be received by
//! running `bspc wm -d` your shell.

use crate::errors::ReplyError;
use crate::socket::BspcCommunication;
use crate::tree::Monitor;
use crate::{socket, Id};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FocusHistoryEntry {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub focused_monitor_id: Id,
    pub primary_monitor_id: Option<Id>,
    pub clients_count: u32,
    pub monitors: Vec<Monitor>,
    pub focus_history: Vec<FocusHistoryEntry>,
    pub stacking_list: Vec<Id>,
}

/// Returns a dump of the current bspwm state <br>
/// Contains all monitors with their desktops (and window respectively)
pub fn get_current_state() -> Result<State, ReplyError> {
    let mut conn = socket::connect()?;
    conn.send_message("wm\x00-d\x00")?;

    let reply = conn.receive_message()?;
    let state: State = serde_json::from_str(&reply[0])?;
    Ok(state)
}

#[cfg(test)]
mod test {
    use std::process::Command;

    use super::*;

    #[test]
    fn parse_state_dump() {
        let state = Command::new("bspc").arg("wm").arg("-d").output().unwrap();
        let state = std::str::from_utf8(&state.stdout).unwrap();

        if state.len() > 1 {
            let tree: State = serde_json::from_str(state).unwrap();
            println!("{:#?}", tree);
        }
    }
}
