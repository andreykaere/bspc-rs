use std::collections::HashMap;
use std::io::BufRead;
use std::io::BufReader;
use std::os::unix::net::UnixStream;
use std::string::ToString;
use strum_macros::Display;

use crate::communication::BspcCommunication;
use crate::errors::{ParseError, ReplyError};
use crate::properties::*;
use crate::{bspc, Id};

#[derive(Display, Debug)]
#[strum(serialize_all = "snake_case")]
pub enum Subscription {
    All,
    Report,
    Monitor,
    Desktop,
    Node,
    MonitorAdd,
    MonitorRename,
    MonitorSwap,
    MonitorFocus,
    MonitorGeometry,
    DesktopAdd,
    DesktopRename,
    DesktopRemove,
    DesktopSwap,
    DesktopTransfer,
    DesktopFocus,
    DesktopActivate,
    DesktopLayout,
    NodeAdd,
    NodeRemove,
    NodeSwap,
    NodeTransfer,
    NodeFocus,
    NodeActivate,
    NodePresel,
    NodeStack,
    NodeGeometry,
    NodeState,
    NodeFlag,
    NodeLayer,
    PointerAction,
}

#[derive(Debug)]
pub struct MonitorAddInfo {
    pub monitor_id: Id,
    pub monitor_name: String,
    pub monitor_geometry: Rectangle,
}

#[derive(Debug)]
pub struct MonitorRenameInfo {
    pub monitor_id: Id,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug)]
pub struct MonitorRemoveInfo {
    pub monitor_id: Id,
}

#[derive(Debug)]
pub struct MonitorSwapInfo {
    pub src_monitor_id: Id,
    pub dst_monitor_id: Id,
}

#[derive(Debug)]
pub struct MonitorFocusInfo {
    pub monitor_id: Id,
}

#[derive(Debug)]
pub struct MonitorGeometryInfo {
    pub monitor_id: Id,
    pub monitor_geometry: Rectangle,
}

#[derive(Debug)]
pub enum MonitorEvent {
    MonitorAdd(MonitorAddInfo),
    MonitorRename(MonitorRenameInfo),
    MonitorRemove(MonitorRemoveInfo),
    MonitorSwap(MonitorSwapInfo),
    MonitorFocus(MonitorFocusInfo),
    MonitorGeometry(MonitorGeometryInfo),
}

#[derive(Debug)]
pub struct DesktopAddInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub desktop_name: String,
}

#[derive(Debug)]
pub struct DesktopRenameInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug)]
pub struct DesktopRemoveInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
}

#[derive(Debug)]
pub struct DesktopSwapInfo {
    pub src_monitor_id: Id,
    pub src_desktop_id: Id,
    pub dst_monitor_id: Id,
    pub dst_desktop_id: Id,
}

#[derive(Debug)]
pub struct DesktopTransferInfo {
    pub src_monitor_id: Id,
    pub src_desktop_id: Id,
    pub dst_monitor_id: Id,
}

#[derive(Debug)]
pub struct DesktopFocusInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
}

#[derive(Debug)]
pub struct DesktopActivateInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
}

#[derive(Debug)]
pub struct DesktopLayoutInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub layout: Layout,
}

#[derive(Debug)]
pub enum DesktopEvent {
    DesktopAdd(DesktopAddInfo),
    DesktopRename(DesktopRenameInfo),
    DesktopRemove(DesktopRemoveInfo),
    DesktopSwap(DesktopSwapInfo),
    DesktopTransfer(DesktopTransferInfo),
    DesktopFocus(DesktopFocusInfo),
    DesktopActivate(DesktopActivateInfo),
    DesktopLayout(DesktopLayoutInfo),
}

#[derive(Debug)]
pub struct NodeAddInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub ip_id: Id,
    pub node_id: Id,
}

#[derive(Debug)]
pub struct NodeRemoveInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
}

#[derive(Debug)]
pub struct NodeSwapInfo {
    pub src_monitor_id: Id,
    pub src_desktop_id: Id,
    pub src_node_id: Id,
    pub dst_monitor_id: Id,
    pub dst_desktop_id: Id,
    pub dst_node_id: Id,
}

#[derive(Debug)]
pub struct NodeTransferInfo {
    pub src_monitor_id: Id,
    pub src_desktop_id: Id,
    pub src_node_id: Id,
    pub dst_monitor_id: Id,
    pub dst_desktop_id: Id,
    pub dst_node_id: Id,
}

#[derive(Debug)]
pub struct NodeFocusInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
}

#[derive(Debug)]
pub struct NodeActivateInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
}

#[derive(Debug)]
pub struct NodePreselInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
    pub presel: Presel,
}

#[derive(Debug)]
pub struct NodeStackInfo {
    pub node_id_1: Id,
    pub stack: Stack,
    pub node_id_2: Id,
}

#[derive(Debug)]
pub struct NodeGeometryInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
    pub node_geometry: Rectangle,
}

#[derive(Debug)]
pub struct NodeStateInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
    pub state: State,
    pub switch: Switch,
}

#[derive(Debug)]
pub struct NodeFlagInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
    pub flag: Flag,
    pub switch: Switch,
}

#[derive(Debug)]
pub struct NodeLayerInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
    pub layer: Layer,
}

#[derive(Debug)]
pub enum NodeEvent {
    NodeAdd(NodeAddInfo),
    NodeRemove(NodeRemoveInfo),
    NodeSwap(NodeSwapInfo),
    NodeTransfer(NodeTransferInfo),
    NodeFocus(NodeFocusInfo),
    NodeActivate(NodeActivateInfo),
    NodePresel(NodePreselInfo),
    NodeStack(NodeStackInfo),
    NodeGeometry(NodeGeometryInfo),
    NodeState(NodeStateInfo),
    NodeFlag(NodeFlagInfo),
    NodeLayer(NodeLayerInfo),
}

#[derive(Debug)]
pub struct PointerActionInfo {
    pub monitor_id: Id,
    pub desktop_id: Id,
    pub node_id: Id,
    pub action: Action,
    pub action_state: ActionState,
}

#[derive(Debug)]
pub enum ReportDesktopState {
    Free,
    Occupied,
    Urgent,
}

#[derive(Debug)]
pub struct ReportDesktopInfo {
    pub state: ReportDesktopState,
    pub focused: bool,
}

#[derive(Debug)]
pub struct ReportMonitorInfo {
    pub desktops: HashMap<String, ReportDesktopInfo>,
    pub focused: bool,
}

#[derive(Debug)]
// TODO: implement it
pub struct ReportInfo(HashMap<String, ReportMonitorInfo>);

#[derive(Debug)]
pub enum Event {
    Report(ReportInfo),
    MonitorEvent(MonitorEvent),
    DesktopEvent(DesktopEvent),
    NodeEvent(NodeEvent),
    PointerAction(PointerActionInfo),
}

#[derive(Debug)]
pub struct EventIterator {
    stream_buf: BufReader<UnixStream>,
}

impl Iterator for EventIterator {
    type Item = Result<Event, ReplyError>;

    fn next(&mut self) -> Option<Self::Item> {
        // for line in self.stream_buf.lines() {
        //     match line {
        let mut reply = String::new();
        let result = self.stream_buf.read_line(&mut reply);

        match result {
            Ok(_) => {
                let event = reply.parse::<Event>().map_err(From::from);
                Some(event)
            }

            Err(e) => Some((Err(e)).map_err(From::from)),
        }
    }
}

/// Subscribes to the given events
pub fn subscribe(
    subscriptions: &[Subscription],
    fifo_flag: bool,
    count: Option<u32>,
) -> Result<EventIterator, ReplyError> {
    let mut conn = bspc::connect()?;

    let all_subscriptions = &subscriptions
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\x00");

    let mut count_option = String::new();
    let mut fifo_option = "";

    if let Some(x) = count {
        count_option = format!("--count\x00{}\x00", x);
    }

    if fifo_flag {
        fifo_option = "--fifo\x00";
    }

    let subscribe_message = format!(
        "subscribe\x00{}{}{}\x00",
        fifo_option, count_option, all_subscriptions
    );

    conn.send_message(&subscribe_message)?;

    Ok(EventIterator {
        stream_buf: BufReader::new(conn),
    })
}
