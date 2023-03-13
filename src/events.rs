use std::collections::{HashMap, VecDeque};
use std::os::unix::net::UnixStream;
use std::string::ToString;
use strum_macros::Display;

use crate::communication::BspcCommunication;
use crate::errors::{ParseError, ReplyError};
use crate::properties::*;
use crate::BspwmConnection;

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
    pub monitor_id: i32,
    pub monitor_name: String,
    pub monitor_geometry: Rectangle,
}

#[derive(Debug)]
pub struct MonitorRenameInfo {
    pub monitor_id: i32,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug)]
pub struct MonitorRemoveInfo {
    pub monitor_id: i32,
}

#[derive(Debug)]
pub struct MonitorSwapInfo {
    pub src_monitor_id: i32,
    pub dst_monitor_id: i32,
}

#[derive(Debug)]
pub struct MonitorFocusInfo {
    pub monitor_id: i32,
}

#[derive(Debug)]
pub struct MonitorGeometryInfo {
    pub monitor_id: i32,
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
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub desktop_name: String,
}

#[derive(Debug)]
pub struct DesktopRenameInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub old_name: String,
    pub new_name: String,
}

#[derive(Debug)]
pub struct DesktopRemoveInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
}

#[derive(Debug)]
pub struct DesktopSwapInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub dst_monitor_id: i32,
    pub dst_desktop_id: i32,
}

#[derive(Debug)]
pub struct DesktopTransferInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub dst_monitor_id: i32,
}

#[derive(Debug)]
pub struct DesktopFocusInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
}

#[derive(Debug)]
pub struct DesktopActivateInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
}

#[derive(Debug)]
pub struct DesktopLayoutInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
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
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub ip_id: i32,
    pub node_id: i32,
}

#[derive(Debug)]
pub struct NodeRemoveInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

#[derive(Debug)]
pub struct NodeSwapInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub src_node_id: i32,
    pub dst_monitor_id: i32,
    pub dst_desktop_id: i32,
    pub dst_node_id: i32,
}

#[derive(Debug)]
pub struct NodeTransferInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub src_node_id: i32,
    pub dst_monitor_id: i32,
    pub dst_desktop_id: i32,
    pub dst_node_id: i32,
}

#[derive(Debug)]
pub struct NodeFocusInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

#[derive(Debug)]
pub struct NodeActivateInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

#[derive(Debug)]
pub struct NodePreselInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub presel: Presel,
}

#[derive(Debug)]
pub struct NodeStackInfo {
    pub node_id_1: i32,
    pub stack: Stack,
    pub node_id_2: i32,
}

#[derive(Debug)]
pub struct NodeGeometryInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub node_geometry: Rectangle,
}

#[derive(Debug)]
pub struct NodeStateInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub state: State,
    pub switch: Switch,
}

#[derive(Debug)]
pub struct NodeFlagInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub flag: Flag,
    pub switch: Switch,
}

#[derive(Debug)]
pub struct NodeLayerInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
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
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
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
    Monitor(MonitorEvent),
    Desktop(DesktopEvent),
    Node(NodeEvent),
    PointerAction(PointerActionInfo),
}

pub struct EventIterator<'a> {
    pub(super) stream: &'a mut UnixStream,

    // This is needed if by one `read` we obtain more, than one events.
    // Whenever it happens, we push these back and take one from the front.
    pub(super) cache: VecDeque<Result<Event, ReplyError>>,
}

impl<'a> Iterator for EventIterator<'a> {
    type Item = Result<Event, ReplyError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stream.receive_message() {
            Ok(reply) => {
                let mut new_cache = VecDeque::new();

                for res in reply.split('\n') {
                    if res.is_empty() {
                        continue;
                    }

                    let event = res.parse::<Event>().map_err(From::from);

                    new_cache.push_back(event);
                }

                self.cache.append(&mut new_cache);
                match self.cache.pop_front() {
                    Some(x) => Some(x),
                    None => Some(Err(ReplyError::ParseError(
                        ParseError::InsufficientData,
                    ))),
                }
            }

            Err(e) => Some(Err(e)),
        }
    }
}

impl BspwmConnection {
    /// Subscribes to the given events
    pub fn subscribe(
        &mut self,
        fifo: Option<()>,
        count: Option<u32>,
        subscriptions: &[Subscription],
    ) -> Result<(), ReplyError> {
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

        if let Some(()) = fifo {
            fifo_option = "--fifo\x00";
        }
        let subscribe_message = format!(
            "subscribe\x00{}{}{}\x00",
            fifo_option, count_option, all_subscriptions
        );

        self.stream.send_message(&subscribe_message)
    }

    /// Listen to the subscriptions
    pub fn listen(&mut self) -> EventIterator {
        EventIterator {
            stream: &mut self.stream,
            cache: VecDeque::new(),
        }
    }
}
