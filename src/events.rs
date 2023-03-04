pub struct Geometry {
    pub x: i32,
    pub y: i32,
    pub height: u32,
    pub width: u32,
}

pub enum Layout {
    Tiled,
    Monocle,
}

pub enum Dir {
    South,
    North,
    West,
    East,
}

pub enum Presel {
    Dir(Dir),
    Ratio(f32),
    Cancel,
}

pub enum Stack {
    Below,
    Above,
}

pub enum State {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen,
}

pub enum Switch {
    On,
    Off,
}

pub enum Flag {
    Hidden,
    Sticky,
    Private,
    Locked,
    Marked,
    Urgent,
}

pub enum Layer {
    Below,
    Normal,
    Above,
}

pub enum Action {
    Move,
    ResizeCorner,
    ResizeSide,
}

pub enum ActionState {
    Begin,
    End,
}

pub struct MonitorAddInfo {
    pub monitor_id: i32,
    pub monitor_name: String,
    pub monitor_geometry: Geometry,
}

pub struct MonitorRenameInfo {
    pub monitor_id: i32,
    pub old_name: String,
    pub new_name: String,
}

pub struct MonitorRemoveInfo {
    pub monitor_id: i32,
}

pub struct MonitorSwapInfo {
    pub src_monitor_id: i32,
    pub dst_monitor_id: i32,
}

pub struct MonitorFocusInfo {
    pub monitor_id: i32,
}

pub struct MonitorGeometryInfo {
    pub monitor_id: i32,
    pub monitor_geometry: Geometry,
}

pub enum MonitorEvent {
    MonitorAdd(MonitorAddInfo),
    MonitorRename(MonitorRenameInfo),
    MonitorRemove(MonitorRemoveInfo),
    MonitorSwap(MonitorSwapInfo),
    MonitorFocus(MonitorFocusInfo),
    MonitorGeometry(MonitorGeometryInfo),
}

pub struct DesktopAddInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub desktop_name: String,
}

pub struct DesktopRenameInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub old_name: String,
    pub new_name: String,
}

pub struct DesktopRemoveInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
}

pub struct DesktopSwapInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub dst_monitor_id: i32,
    pub dst_desktop_id: i32,
}

pub struct DesktopTransferInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub dst_monitor_id: i32,
}

pub struct DesktopFocusInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
}

pub struct DesktopActivateInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
}

pub struct DesktopLayoutInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub layout: Layout,
}

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

pub struct NodeAddInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub ip_id: i32,
    pub node_id: i32,
}

pub struct NodeRemoveInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

pub struct NodeSwapInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub src_node_id: i32,
    pub dst_monitor_id: i32,
    pub dst_desktop_id: i32,
    pub dst_node_id: i32,
}

pub struct NodeTransferInfo {
    pub src_monitor_id: i32,
    pub src_desktop_id: i32,
    pub src_node_id: i32,
    pub dst_monitor_id: i32,
    pub dst_desktop_id: i32,
    pub dst_node_id: i32,
}

pub struct NodeFocusInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

pub struct NodeActivateInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
}

pub struct NodePreselInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub presel: Presel,
}

pub struct NodeStackInfo {
    pub node_id_1: i32,
    pub stack: Stack,
    pub node_id_2: i32,
}

pub struct NodeGeometryInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub node_geometry: Geometry,
}

pub struct NodeStateInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub state: State,
    pub switch: Switch,
}

pub struct NodeFlagInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub flag: Flag,
    pub switch: Switch,
}

pub struct NodeLayerInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub layer: Layer,
}

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

pub struct PointerActionInfo {
    pub monitor_id: i32,
    pub desktop_id: i32,
    pub node_id: i32,
    pub action: Action,
    pub action_state: ActionState,
}

// TODO: implement it
pub struct ReportInfo {}

pub enum Event {
    Report(ReportInfo),
    Monitor(MonitorEvent),
    Dekstop(DesktopEvent),
    Node(NodeEvent),
    PointerAction(PointerActionInfo),
}
