#[derive(Debug)]
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
pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}

#[derive(Debug)]
pub enum Layout {
    Tiled,
    Monocle,
}

#[derive(Debug)]
pub enum Dir {
    South,
    North,
    West,
    East,
}

#[derive(Debug)]
pub enum Presel {
    Dir(Dir),
    Ratio(f32),
    Cancel,
}

#[derive(Debug)]
pub enum Stack {
    Below,
    Above,
}

#[derive(Debug)]
pub enum State {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen,
}

#[derive(Debug)]
pub enum Switch {
    On,
    Off,
}

#[derive(Debug)]
pub enum Flag {
    Hidden,
    Sticky,
    Private,
    Locked,
    Marked,
    Urgent,
}

#[derive(Debug)]
pub enum Layer {
    Below,
    Normal,
    Above,
}

#[derive(Debug)]
pub enum Action {
    Move,
    ResizeCorner,
    ResizeSide,
}

#[derive(Debug)]
pub enum ActionState {
    Begin,
    End,
}

#[derive(Debug)]
pub struct MonitorAddInfo {
    pub monitor_id: i32,
    pub monitor_name: String,
    pub monitor_geometry: Geometry,
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
    pub monitor_geometry: Geometry,
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
    pub node_geometry: Geometry,
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
// TODO: implement it
pub struct ReportInfo {}

#[derive(Debug)]
pub enum Event {
    Report(ReportInfo),
    Monitor(MonitorEvent),
    Dekstop(DesktopEvent),
    Node(NodeEvent),
    PointerAction(PointerActionInfo),
}
