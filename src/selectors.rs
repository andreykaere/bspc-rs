use crate::properties::{CycleDir, Dir};

pub struct MonitorSelector {}
pub struct DesktopSelector {}

pub enum PathJump {
    First,
    Second,
    Brother,
    Parent,
    Dir(Dir),
}

pub struct Path {
    pub root: DesktopSelector,
    pub jumps: Vec<PathJump>,
}

pub enum NodeDescriptor {
    Dir(Dir),
    CycleDir(CycleDir),
    Path(Path),
    Any,
    FirstAncestor,
    Last,
    Newest,
    Older,
    Newer,
    Focused,
    Pointer,
    Biggest,
    Smallest,
    NodeId(i32),
}

pub struct NodeModifier;

pub struct NodeSelector {
    pub reference: Option<Box<NodeSelector>>,
    pub descriptor: NodeDescriptor,
    pub modifier: Option<NodeModifier>,
}

pub enum Selector {
    Monitor(MonitorSelector),
    Desktop(DesktopSelector),
    Node(NodeSelector),
}
