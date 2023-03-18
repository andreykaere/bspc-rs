// use crate::properties::{CycleDir, Dir, Flag, Layer, SplitType, State};

// pub struct MonitorSelector {}
// pub struct DesktopSelector {}

// pub enum PathJump {
//     First,
//     Second,
//     Brother,
//     Parent,
//     Dir(Dir),
// }

// pub struct Path {
//     pub root: DesktopSelector,
//     pub jumps: Vec<PathJump>,
// }

// pub enum NodeDescriptor {
//     Dir(Dir),
//     CycleDir(CycleDir),
//     Path(Path),
//     Any,
//     FirstAncestor,
//     Last,
//     Newest,
//     Older,
//     Newer,
//     Focused,
//     Pointer,
//     Biggest,
//     Smallest,
//     NodeId(i32),
// }

// pub struct NodeModifier {
//     focused: Option<bool>,
//     active: Option<bool>,
//     automatic: Option<bool>,
//     local: Option<bool>,
//     leaf: Option<bool>,
//     window: Option<bool>,
//     state: Option<State>,
//     flag: Option<Flag>,
//     layer: Option<Layer>,
//     split_type: Option<SplitType>,
//     same_class: Option<bool>,
//     descendant_of: Option<bool>,
//     ancestor_of: Option<bool>,
// }

// // TODO: make a selector and add a check at compile time for validating if
// // the request makes sense (is correct)
// pub struct NodeSelector {
//     pub reference: Option<Box<NodeSelector>>,
//     pub descriptor: NodeDescriptor,
//     pub modifier: Option<NodeModifier>,
// }

pub struct NodeSelector<'a>(pub &'a str);
pub struct DesktopSelector<'a>(pub &'a str);
pub struct MonitorSelector<'a>(pub &'a str);

pub trait Selector {
    /// Checks if given selector is valid
    fn is_valid(&self) -> bool;

    /// Extracts selector from wrapper
    fn extract(&self) -> &str;

    /// Returns kind of selector, i.e. Node, Desktop or Monitor
    fn kind(&self) -> &str;
}

impl<'a> Selector for NodeSelector<'a> {
    // TODO
    fn is_valid(&self) -> bool {
        true
    }

    fn extract(&self) -> &str {
        self.0
    }

    fn kind(&self) -> &str {
        "Node"
    }
}

impl<'a> Selector for DesktopSelector<'a> {
    // TODO
    fn is_valid(&self) -> bool {
        true
    }

    fn extract(&self) -> &str {
        self.0
    }

    fn kind(&self) -> &str {
        "Desktop"
    }
}

impl<'a> Selector for MonitorSelector<'a> {
    // TODO
    fn is_valid(&self) -> bool {
        true
    }

    fn extract(&self) -> &str {
        self.0
    }

    fn kind(&self) -> &str {
        "Monitor"
    }
}
