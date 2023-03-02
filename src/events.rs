struct Geometry {
    x: i32,
    y: i32,
    height: u32,
    width: u32,
}

enum Layout {
    Tiled,
    Monocle,
}

enum Dir {
    South,
    North,
    West,
    East,
}

enum Presel {
    Dir(Dir),
    Ratio(f32),
    Cancel,
}

enum Stack {
    Below,
    Above,
}

enum State {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen,
}

enum Switch {
    On,
    Off,
}

enum Flag {
    Hidden,
    Sticky,
    Private,
    Locked,
    Marked,
    Urgent,
}

enum Layer {
    Below,
    Normal,
    Above,
}

enum Action {
    Move,
    ResizeCorner,
    ResizeSide,
}

enum ActionState {
    Begin,
    End,
}

pub struct MonitorAddInfo {
    monitor_id: i32,
    monitor_name: String,
    monitor_geometry: Geometry,
}

pub struct MonitorRenameInfo {
    monitor_id: i32,
    old_name: String,
    new_name: String,
}

pub struct MonitorRemoveInfo {
    monitor_id: i32,
}

pub struct MonitorSwapInfo {
    src_monitor_id: i32,
    dst_monitor_id: i32,
}

pub struct MonitorFocusInfo {
    monitor_id: i32,
}

pub struct MonitorGeometryInfo {
    monitor_id: i32,
    monitor_geometry: Geometry,
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
    monitor_id: i32,
    desktop_id: i32,
    desktop_name: String,
}

pub struct DesktopRenameInfo {
    monitor_id: i32,
    desktop_id: i32,
    old_name: String,
    new_name: String,
}

pub struct DesktopRemoveInfo {
    monitor_id: i32,
    desktop_id: i32,
}

pub struct DesktopSwapInfo {
    src_monitor_id: i32,
    src_desktop_id: i32,
    dst_monitor_id: i32,
    dst_desktop_id: i32,
}

pub struct DesktopTransferInfo {
    src_monitor_id: i32,
    src_desktop_id: i32,
    dst_monitor_id: i32,
}

pub struct DesktopFocusInfo {
    monitor_id: i32,
    desktop_id: i32,
}

pub struct DesktopActivateInfo {
    monitor_id: i32,
    desktop_id: i32,
}

pub struct DesktopLayoutInfo {
    monitor_id: i32,
    desktop_id: i32,
    layout: Layout,
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
    monitor_id: i32,
    desktop_id: i32,
    ip_id: i32,
    node_id: i32,
}

pub struct NodeRemoveInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
}

pub struct NodeSwapInfo {
    src_monitor_id: i32,
    src_desktop_id: i32,
    src_node_id: i32,
    dst_monitor_id: i32,
    dst_desktop_id: i32,
    dst_node_id: i32,
}

pub struct NodeTransferInfo {
    src_monitor_id: i32,
    src_desktop_id: i32,
    src_node_id: i32,
    dst_monitor_id: i32,
    dst_desktop_id: i32,
    dst_node_id: i32,
}

pub struct NodeFocusInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
}

pub struct NodeActivateInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
}

pub struct NodePreselInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
    presel: Presel,
}

pub struct NodeStackInfo {
    node_id_1: i32,
    stack: Stack,
    node_id_2: i32,
}

pub struct NodeGeometryInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
    node_geometry: Geometry,
}

pub struct NodeStateInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
    state: State,
    switch: Switch,
}

pub struct NodeFlagInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
    flag: Flag,
    switch: Switch,
}

pub struct NodeLayerInfo {
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
    layer: Layer,
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
    monitor_id: i32,
    desktop_id: i32,
    node_id: i32,
    action: Action,
    action_state: ActionState,
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
