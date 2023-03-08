use super::events::{Dir, Layer, Layout, State};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct Padding {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32,
}

#[derive(Debug)]
pub enum SplitType {
    Vertical,
    Horizontal,
}

#[derive(Debug)]
pub struct Client {
    pub class_name: String,
    pub instance_name: String,
    pub border_with: i32,
    pub state: State,
    pub last_state: State,
    pub layer: Layer,
    pub last_layer: Layer,
    pub urgent: bool,
    pub shown: bool,
    pub tiled_rectangle: Rectangle,
    pub floating_rectangle: Rectangle,
}

#[derive(Debug)]
pub struct Constraints {
    pub min_width: i32,
    pub min_height: i32,
}

#[derive(Debug)]
pub struct PreselNode {
    pub split_dir: Dir,
    pub split_ration: f32,
}

#[derive(Debug)]
pub struct Node {
    pub id: i32,
    pub split_type: SplitType,
    pub split_ration: f32,
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
    pub client: Client,
}

#[derive(Debug)]
pub struct Desktop {
    pub name: String,
    pub id: i32,
    pub layout: Layout,
    pub user_layout: Layout,
    pub window_gap: i32,
    pub padding: Padding,
    pub root: Option<Node>,
}

#[derive(Debug)]
pub struct Monitor {
    pub name: String,
    pub id: i32,
    pub randr_id: i32,
    pub wired: bool,
    pub sticky_count: i32,
    pub window_gap: i32,
    pub border_with: i32,
    pub focused_desktop_id: i32,
    pub padding: Padding,
    pub rectangle: Rectangle,
}

#[derive(Debug)]
pub struct Tree {
    pub monitors: HashMap<String, Monitor>,
}
