use serde::{Deserialize, Serialize};

use crate::errors::ReplyError;
use crate::properties::{Dir, Layer, Layout, Rectangle, SplitType, State};
use crate::Id;

#[derive(Debug, Serialize, Deserialize)]
pub struct Padding {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Client {
    pub class_name: String,
    pub instance_name: String,
    pub border_width: i32,
    pub state: State,
    pub last_state: State,
    pub layer: Layer,
    pub last_layer: Layer,
    pub urgent: bool,
    pub shown: bool,
    pub tiled_rectangle: Rectangle,
    pub floating_rectangle: Rectangle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Constraints {
    pub min_width: i32,
    pub min_height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreselNode {
    pub split_dir: Dir,
    pub split_ratio: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: Id,
    pub split_type: SplitType,
    pub split_ratio: f32,
    pub vacant: bool,
    pub hidden: bool,
    pub sticky: bool,
    pub private: bool,
    pub locked: bool,
    pub marked: bool,
    pub presel: Option<PreselNode>,
    pub rectangle: Rectangle,
    pub constraints: Constraints,
    pub first_child: Option<Box<Node>>,
    pub second_child: Option<Box<Node>>,
    pub client: Option<Client>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Desktop {
    pub name: String,
    pub id: Id,
    pub layout: Layout,
    pub user_layout: Layout,
    pub window_gap: i32,
    pub border_width: i32,
    pub focused_node_id: Id,
    pub padding: Padding,
    pub root: Option<Node>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Monitor {
    pub name: String,
    pub id: Id,
    pub randr_id: Id,
    pub wired: bool,
    pub sticky_count: i32,
    pub window_gap: i32,
    pub border_width: i32,
    pub focused_desktop_id: Id,
    pub padding: Padding,
    pub rectangle: Rectangle,
    pub desktops: Vec<Desktop>,
}

#[derive(Debug)]
pub enum Tree {
    Node(Node),
    Desktop(Desktop),
    Monitor(Monitor),
}

/// Converts id of the monitor to `Monitor` structure. Returns `None` if there
/// is no monitor with the given id.
pub fn from_id_to_monitor(id: Id) -> Result<Option<Monitor>, ReplyError> {
    todo!();
}

/// Converts id of the desktop to `Desktop` structure. Returns `None` if there
/// is no desktop with the given id.
pub fn from_id_to_desktop(id: Id) -> Result<Option<Desktop>, ReplyError> {
    todo!();
}

/// Converts id of the node to `Node` structure. Returns `None` if there
/// is no node with the given id.
pub fn from_id_to_node(id: Id) -> Result<Option<Node>, ReplyError> {
    todo!();

    // let tree_raw = self.query_tree(QueryOptions::Monitor)?;
    // let tree = if let Tree::Monitor(mon) = tree_raw {
    //     mon
    // } else {
    //     unreachable!();
    // };

    // for desktop in tree.desktops {
    //     let root = desktop.root;

    //     if let Some(root) = root {
    //         while let None = root.client {
    //             if let Some(first_child) = root.first_child {
    //                 if first_child.id == id {
    //                     return Ok(Some(*first_child));
    //                 }
    //             }

    //             if let Some(second_child) = root.second_child {
    //                 if second_child.id == id {
    //                     return Ok(Some(*second_child));
    //                 }
    //             }
    //         }
    //     }
    // }

    // Ok(None)
}

#[cfg(test)]
mod test {
    use std::process::Command;
    use std::thread;
    use std::time::Duration;

    use super::*;
    use crate::events::*;
    use crate::query;
    use crate::selectors::NodeSelector;

    #[test]
    fn parse_tree() {
        let monitor = Command::new("bspc")
            .arg("query")
            .arg("-T")
            .arg("-m")
            .output()
            .unwrap();
        let monitor = std::str::from_utf8(&monitor.stdout).unwrap();

        if monitor.len() > 1 {
            let tree: Monitor = serde_json::from_str(monitor).unwrap();
            println!("{:#?}", tree);
        }
    }

    #[test]
    fn test_from_id_to_node() {
        let window_id = query::query_nodes(
            None,
            None,
            None,
            Some(NodeSelector(".fullscreen")),
        );

        match window_id {
            Ok(_) => {}
            Err(ReplyError::RequestFailed(reply)) => {
                if reply.len() > 0 {
                    panic!("{}", reply);
                }
            }
            Err(e) => {
                panic!("{}", e);
            }
        }
    }
}
