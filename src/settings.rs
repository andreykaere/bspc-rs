//! This module is dedicated to various settings of bspwm. Its functions are
//! equivalent to those, which are run by `bspc config <setting>` command in
//! your shell.

use std::string::ToString;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::errors::{ParseError, ReplyError};
use crate::selectors::{DesktopSelector, MonitorSelector, NodeSelector};
use crate::socket::{self, BspcCommunication};

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Scheme {
    LongestSide,
    Alternate,
    Spiral,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Polarity {
    FirstChild,
    SecondChild,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum Tightness {
    High,
    Low,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum PointerModifier {
    Shift,
    Control,
    Lock,
    Mod1,
    Mod2,
    Mod3,
    Mod4,
    Mod5,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum PointerAction {
    Move,
    ResizeSide,
    ResizeCorner,
    Focus,
    None,
}

#[derive(Debug, Clone, Copy, EnumString, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ClickToFocus {
    Button1,
    Button2,
    Button3,
    Any,
    None,
}

// Template for generating function for handling events
// pub struct Settings {
//     normal_border_color: String,
//     active_border_color: String,
//     focused_border_color: String,
//     presel_feedback_color: String,
//     split_ratio: f32,
//     status_prefix: String,
//     external_rules_command: String,
//     automatic_scheme: Scheme,
//     initial_polarity: Polarity,
//     directional_focus_tightness: Tightness,
//     removal_adjustment: bool,
//     presel_feedback: bool,
//     borderless_monocle: bool,
//     gapless_monocle: bool,
//     top_monocle_padding: i16,
//     right_monocle_padding: i16,
//     bottom_monocle_padding: i16,
//     left_monocle_padding: i16,
//     single_monocle: bool,
//     pointer_motion_interval: u16,
//     pointer_modifier: PointerModifier,
//     pointer_action1: PointerAction,
//     pointer_action2: PointerAction,
//     pointer_action3: PointerAction,
//     click_to_focus: ClickToFocus,
//     swallow_first_click: bool,
//     focus_follows_pointer: bool,
//     pointer_follows_focus: bool,
//     pointer_follows_monitor: bool,
//     mapping_events_count: i32,
//     ignore_ewmh_focus: bool,
//     ignore_ewmh_fullscreen: bool,
//     ignore_ewmh_struts: bool,
//     center_pseudo_tiled: bool,
//     honor_size_hints: bool,
//     remove_disabled_monitors: bool,
//     remove_unplugged_monitors: bool,
//     merge_overlapping_monitors: bool,
// }

trait ConfigProperties {
    fn get_config_property(
        &mut self,
        property: &str,
    ) -> Result<String, ReplyError>;

    fn set_config_property(
        &mut self,
        property: &str,
        value: &str,
    ) -> Result<(), ReplyError>;
}

impl<T: BspcCommunication> ConfigProperties for T {
    fn get_config_property(
        &mut self,
        property: &str,
    ) -> Result<String, ReplyError> {
        self.send_message(&format!("config\x00{}\x00", property))?;
        let reply = self.receive_message()?;

        if reply.len() > 1 {
            // TODO: Test if this can happen
            panic!("{}", format!("Something is weird, reply has more than one element, this is debug log: {:#?}", reply));
        }

        let reply = &reply[0];

        Ok(reply.to_string())
    }

    fn set_config_property(
        &mut self,
        property: &str,
        value: &str,
    ) -> Result<(), ReplyError> {
        self.send_message(&format!("config\x00{}\x00{}\x00", property, value))?;
        let reply = self.receive_message()?;

        if reply.is_empty() {
            return Ok(());
        }

        Err(ParseError::ConversionFailed)?
    }
}

pub fn get_normal_border_color() -> Result<String, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("normal_border_color")
}

pub fn get_active_border_color() -> Result<String, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("active_border_color")
}

pub fn get_focused_border_color() -> Result<String, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("focused_border_color")
}

pub fn get_presel_feedback_color() -> Result<String, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("presel_feedback_color")
}

pub fn get_split_ratio() -> Result<f32, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("split_ratio")?
        .parse()
        .map_err(From::from)
}

pub fn get_status_prefix() -> Result<String, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("status_prefix")
}

pub fn get_external_rules_command() -> Result<String, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("external_rules_command")
}

pub fn get_automatic_scheme() -> Result<Scheme, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("automatic_scheme")?
        .parse()
        .map_err(From::from)
}

pub fn get_initial_polarity() -> Result<Polarity, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("initial_polarity")?
        .parse()
        .map_err(From::from)
}

pub fn get_directional_focus_tightness() -> Result<Tightness, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("directional_focus_tightness")?
        .parse()
        .map_err(From::from)
}

pub fn get_removal_adjustment() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("removal_adjustment")?
        .parse()
        .map_err(From::from)
}

pub fn get_presel_feedback() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("presel_feedback")?
        .parse()
        .map_err(From::from)
}

pub fn get_borderless_monocle() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("borderless_monocle")?
        .parse()
        .map_err(From::from)
}

pub fn get_gapless_monocle() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("gapless_monocle")?
        .parse()
        .map_err(From::from)
}

pub fn get_top_monocle_padding() -> Result<i16, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("top_monocle_padding")?
        .parse()
        .map_err(From::from)
}

pub fn get_right_monocle_padding() -> Result<i16, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("right_monocle_padding")?
        .parse()
        .map_err(From::from)
}

pub fn get_bottom_monocle_padding() -> Result<i16, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("bottom_monocle_padding")?
        .parse()
        .map_err(From::from)
}

pub fn get_left_monocle_padding() -> Result<i16, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("left_monocle_padding")?
        .parse()
        .map_err(From::from)
}

pub fn get_single_monocle() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("single_monocle")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_motion_interval() -> Result<u16, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_motion_interval")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_modifier() -> Result<PointerModifier, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_modifier")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_action1() -> Result<PointerAction, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_action1")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_action2() -> Result<PointerAction, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_action2")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_action3() -> Result<PointerAction, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_action3")?
        .parse()
        .map_err(From::from)
}

pub fn get_click_to_focus() -> Result<ClickToFocus, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("click_to_focus")?
        .parse()
        .map_err(From::from)
}

pub fn get_swallow_first_click() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("swallow_first_click")?
        .parse()
        .map_err(From::from)
}

pub fn get_focus_follows_pointer() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("focus_follows_pointer")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_follows_focus() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_follows_focus")?
        .parse()
        .map_err(From::from)
}

pub fn get_pointer_follows_monitor() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("pointer_follows_monitor")?
        .parse()
        .map_err(From::from)
}

pub fn get_mapping_events_count() -> Result<i32, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("mapping_events_count")?
        .parse()
        .map_err(From::from)
}

pub fn get_ignore_ewmh_focus() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("ignore_ewmh_focus")?
        .parse()
        .map_err(From::from)
}

pub fn get_ignore_ewmh_fullscreen() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("ignore_ewmh_fullscreen")?
        .parse()
        .map_err(From::from)
}

pub fn get_ignore_ewmh_struts() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("ignore_ewmh_struts")?
        .parse()
        .map_err(From::from)
}

pub fn get_center_pseudo_tiled() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("center_pseudo_tiled")?
        .parse()
        .map_err(From::from)
}

pub fn get_honor_size_hints() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("honor_size_hints")?
        .parse()
        .map_err(From::from)
}

pub fn get_remove_disabled_monitors() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("remove_disabled_monitors")?
        .parse()
        .map_err(From::from)
}

pub fn get_remove_unplugged_monitors() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("remove_unplugged_monitors")?
        .parse()
        .map_err(From::from)
}

pub fn get_merge_overlapping_monitors() -> Result<bool, ReplyError> {
    let mut conn = socket::connect()?;

    conn.get_config_property("merge_overlapping_monitors")?
        .parse()
        .map_err(From::from)
}

pub fn set_normal_border_color(value: String) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("normal_border_color", &value)
}

pub fn set_active_border_color(value: String) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("active_border_color", &value)
}

pub fn set_focused_border_color(value: String) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("focused_border_color", &value)
}

pub fn set_presel_feedback_color(value: String) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("presel_feedback_color", &value)
}

pub fn set_split_ratio(value: f32) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("split_ratio", &value.to_string())
}

pub fn set_status_prefix(value: String) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("status_prefix", &value)
}

pub fn set_external_rules_command(value: String) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("external_rules_command", &value)
}

pub fn set_automatic_scheme(value: Scheme) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("automatic_scheme", &value.to_string())
}

pub fn set_initial_polarity(value: Polarity) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("initial_polarity", &value.to_string())
}

pub fn set_directional_focus_tightness(
    value: Tightness,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("directional_focus_tightness", &value.to_string())
}

pub fn set_removal_adjustment(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("removal_adjustment", &value.to_string())
}

pub fn set_presel_feedback(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("presel_feedback", &value.to_string())
}

pub fn set_borderless_monocle(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("borderless_monocle", &value.to_string())
}

pub fn set_gapless_monocle(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("gapless_monocle", &value.to_string())
}

pub fn set_top_monocle_padding(value: i16) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("top_monocle_padding", &value.to_string())
}

pub fn set_right_monocle_padding(value: i16) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("right_monocle_padding", &value.to_string())
}

pub fn set_bottom_monocle_padding(value: i16) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("bottom_monocle_padding", &value.to_string())
}

pub fn set_left_monocle_padding(value: i16) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("left_monocle_padding", &value.to_string())
}

pub fn set_single_monocle(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("single_monocle", &value.to_string())
}

pub fn set_pointer_motion_interval(value: u16) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_motion_interval", &value.to_string())
}

pub fn set_pointer_modifier(value: PointerModifier) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_modifier", &value.to_string())
}

pub fn set_pointer_action1(value: PointerAction) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_action1", &value.to_string())
}

pub fn set_pointer_action2(value: PointerAction) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_action2", &value.to_string())
}

pub fn set_pointer_action3(value: PointerAction) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_action3", &value.to_string())
}

pub fn set_click_to_focus(value: ClickToFocus) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("click_to_focus", &value.to_string())
}

pub fn set_swallow_first_click(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("swallow_first_click", &value.to_string())
}

pub fn set_focus_follows_pointer(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("focus_follows_pointer", &value.to_string())
}

pub fn set_pointer_follows_focus(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_follows_focus", &value.to_string())
}

pub fn set_pointer_follows_monitor(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("pointer_follows_monitor", &value.to_string())
}

pub fn set_mapping_events_count(value: i32) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("mapping_events_count", &value.to_string())
}

pub fn set_ignore_ewmh_focus(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("ignore_ewmh_focus", &value.to_string())
}

pub fn set_ignore_ewmh_fullscreen(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("ignore_ewmh_fullscreen", &value.to_string())
}

pub fn set_ignore_ewmh_struts(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("ignore_ewmh_struts", &value.to_string())
}

pub fn set_center_pseudo_tiled(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("center_pseudo_tiled", &value.to_string())
}

pub fn set_honor_size_hints(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("honor_size_hints", &value.to_string())
}

pub fn set_remove_disabled_monitors(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("remove_disabled_monitors", &value.to_string())
}

pub fn set_remove_unplugged_monitors(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("remove_unplugged_monitors", &value.to_string())
}

pub fn set_merge_overlapping_monitors(value: bool) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    conn.set_config_property("merge_overlapping_monitors", &value.to_string())
}

/// Sets `border_width` for nodes, that satisfy given conditions.
pub fn set_border_width(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    node_selector: Option<NodeSelector>,
    border_width: i32,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    todo!();
}

/// Sets `window_gap` for desktops, that satisfy given conditions.
pub fn set_window_gap(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    window_gap: i32,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    todo!();
}

/// Sets `top_padding` for desktops/monitors, that satisfy given conditions.
pub fn top_padding(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    top_padding: i32,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    todo!();
}

/// Sets `right_padding` for deskrights/monitors, that satisfy given conditions.
pub fn right_padding(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    right_padding: i32,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    todo!();
}

/// Sets `bottom_padding` for deskbottoms/monitors, that satisfy given conditions.
pub fn bottom_padding(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    bottom_padding: i32,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    todo!();
}

/// Sets `left_padding` for desklefts/monitors, that satisfy given conditions.
pub fn left_padding(
    monitor_selector: Option<MonitorSelector>,
    desktop_selector: Option<DesktopSelector>,
    left_padding: i32,
) -> Result<(), ReplyError> {
    let mut conn = socket::connect()?;

    todo!();
}
