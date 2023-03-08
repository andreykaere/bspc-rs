use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Layout {
    Tiled,
    Monocle,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Flag {
    Hidden,
    Sticky,
    Private,
    Locked,
    Marked,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
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
