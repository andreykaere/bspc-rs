use serde::{Deserialize, Serialize};
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, Serialize, Deserialize)]
pub struct Rectangle {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Layout {
    Tiled,
    Monocle,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Dir {
    South,
    North,
    West,
    East,
}

#[derive(Debug, EnumString, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum CycleDir {
    Next,
    Prev,
}

#[derive(Debug)]
pub enum Presel {
    Dir(Dir),
    Ratio(f32),
    Cancel,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Stack {
    Below,
    Above,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum State {
    Tiled,
    PseudoTiled,
    Floating,
    Fullscreen,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Switch {
    On,
    Off,
}

#[derive(Debug, Serialize, Deserialize, EnumString)]
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

#[derive(Debug, Serialize, Deserialize, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Layer {
    Below,
    Normal,
    Above,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Action {
    Move,
    ResizeCorner,
    ResizeSide,
}

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ActionState {
    Begin,
    End,
}
