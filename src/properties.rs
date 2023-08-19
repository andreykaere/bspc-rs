use serde::{Deserialize, Serialize};
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, EnumString, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Layout {
    Tiled,
    Monocle,
}

#[derive(Debug, EnumString, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Dir {
    South,
    North,
    West,
    East,
}

#[derive(Debug, EnumString, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum CycleDir {
    Next,
    Prev,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SplitType {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone)]
pub enum Presel {
    Dir(Dir),
    Ratio(f32),
    Cancel,
}

#[derive(Debug, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Stack {
    Below,
    Above,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen,
}

#[derive(Debug, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Switch {
    On,
    Off,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Flag {
    Hidden,
    Sticky,
    Private,
    Locked,
    Marked,
    Urgent,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Clone)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Layer {
    Below,
    Normal,
    Above,
}

#[derive(Debug, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    Move,
    ResizeCorner,
    ResizeSide,
}

#[derive(Debug, EnumString, Clone)]
#[strum(serialize_all = "snake_case")]
pub enum ActionState {
    Begin,
    End,
}
