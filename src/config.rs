use std::os::unix::net::UnixStream;
use std::str::FromStr;
use std::string::ToString;
use strum_macros::Display;
use strum_macros::EnumString;

use crate::errors::{ParseError, ReplyError};
use crate::{BspcCommunication, BspwmConnection};

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

pub struct Config {
    normal_border_color: String,
    active_border_color: String,
    focused_border_color: String,
    presel_feedback_color: String,
    split_ratio: f32,
    status_prefix: String,
    external_rules_command: String,
    automatic_scheme: Scheme,
    initial_polarity: Polarity,
    directional_focus_tightness: Tightness,
    removal_adjustment: bool,
    presel_feedback: bool,
    borderless_monocle: bool,
    gapless_monocle: bool,
    top_monocle_padding: i16,
    right_monocle_padding: i16,
    bottom_monocle_padding: i16,
    left_monocle_padding: i16,
    single_monocle: bool,
    pointer_motion_interval: u16,
    pointer_modifier: PointerModifier,
    pointer_action1: PointerAction,
    pointer_action2: PointerAction,
    pointer_action3: PointerAction,
    click_to_focus: ClickToFocus,
    swallow_first_click: bool,
    focus_follows_pointer: bool,
    pointer_follows_focus: bool,
    pointer_follows_monitor: bool,
    mapping_events_count: i32, // No idea what it does, so unsure about type
    ignore_ewmh_focus: bool,
    ignore_ewmh_fullscreen: bool,
    ignore_ewmh_struts: bool,
    center_pseudo_tiled: bool,
    honor_size_hints: bool,
    remove_disabled_monitors: bool,
    remove_unplugged_monitors: bool,
    merge_overlapping_monitors: bool,
}

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

        Ok(reply)
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

impl Config {
    pub fn load(steam: &mut UnixStream) -> Result<Self, ReplyError> {
        todo!();
    }
}

impl BspwmConnection {
    pub fn get_normal_border_color(&mut self) -> Result<&str, ReplyError> {
        let value = self.get_config_property("normal_border_color")?;
        self.config.normal_border_color = value;

        Ok(&self.config.normal_border_color)
    }

    pub fn get_active_border_color(&mut self) -> Result<&str, ReplyError> {
        let value = self.get_config_property("active_border_color")?;
        self.config.active_border_color = value;

        Ok(&self.config.active_border_color)
    }

    pub fn get_focused_border_color(&mut self) -> Result<&str, ReplyError> {
        let value = self.get_config_property("focused_border_color")?;
        self.config.focused_border_color = value;

        Ok(&self.config.focused_border_color)
    }

    pub fn get_presel_feedback_color(&mut self) -> Result<&str, ReplyError> {
        let value = self.get_config_property("presel_feedback_color")?;
        self.config.presel_feedback_color = value;

        Ok(&self.config.presel_feedback_color)
    }

    pub fn get_split_ratio(&mut self) -> Result<f32, ReplyError> {
        let value = self.get_config_property("split_ratio")?;
        self.config.split_ratio = value.parse()?;

        Ok(self.config.split_ratio)
    }

    pub fn get_status_prefix(&mut self) -> Result<&str, ReplyError> {
        let value = self.get_config_property("status_prefix")?;
        self.config.status_prefix = value;

        Ok(&self.config.status_prefix)
    }

    pub fn get_external_rules_command(&mut self) -> Result<&str, ReplyError> {
        let value = self.get_config_property("external_rules_command")?;
        self.config.external_rules_command = value;

        Ok(&self.config.external_rules_command)
    }

    pub fn get_automatic_scheme(&mut self) -> Result<Scheme, ReplyError> {
        let value = self.get_config_property("automatic_scheme")?;
        self.config.automatic_scheme = value.parse()?;

        Ok(self.config.automatic_scheme)
    }

    pub fn get_initial_polarity(&mut self) -> Result<Polarity, ReplyError> {
        let value = self.get_config_property("initial_polarity")?;
        self.config.initial_polarity = value.parse()?;

        Ok(self.config.initial_polarity)
    }

    pub fn get_directional_focus_tightness(
        &mut self,
    ) -> Result<Tightness, ReplyError> {
        let value = self.get_config_property("directional_focus_tightness")?;
        self.config.directional_focus_tightness = value.parse()?;

        Ok(self.config.directional_focus_tightness)
    }

    pub fn get_removal_adjustment(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("removal_adjustment")?;
        self.config.removal_adjustment = value.parse()?;

        Ok(self.config.removal_adjustment)
    }

    pub fn get_presel_feedback(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("presel_feedback")?;
        self.config.presel_feedback = value.parse()?;

        Ok(self.config.presel_feedback)
    }

    pub fn get_borderless_monocle(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("borderless_monocle")?;
        self.config.borderless_monocle = value.parse()?;

        Ok(self.config.borderless_monocle)
    }

    pub fn get_gapless_monocle(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("gapless_monocle")?;
        self.config.gapless_monocle = value.parse()?;

        Ok(self.config.gapless_monocle)
    }

    pub fn get_top_monocle_padding(&mut self) -> Result<i16, ReplyError> {
        let value = self.get_config_property("top_monocle_padding")?;
        self.config.top_monocle_padding = value.parse()?;

        Ok(self.config.top_monocle_padding)
    }

    pub fn get_right_monocle_padding(&mut self) -> Result<i16, ReplyError> {
        let value = self.get_config_property("right_monocle_padding")?;
        self.config.right_monocle_padding = value.parse()?;

        Ok(self.config.right_monocle_padding)
    }

    pub fn get_bottom_monocle_padding(&mut self) -> Result<i16, ReplyError> {
        let value = self.get_config_property("bottom_monocle_padding")?;
        self.config.bottom_monocle_padding = value.parse()?;

        Ok(self.config.bottom_monocle_padding)
    }

    pub fn get_left_monocle_padding(&mut self) -> Result<i16, ReplyError> {
        let value = self.get_config_property("left_monocle_padding")?;
        self.config.left_monocle_padding = value.parse()?;

        Ok(self.config.left_monocle_padding)
    }

    pub fn get_single_monocle(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("single_monocle")?;
        self.config.single_monocle = value.parse()?;

        Ok(self.config.single_monocle)
    }

    pub fn get_pointer_motion_interval(&mut self) -> Result<u16, ReplyError> {
        let value = self.get_config_property("pointer_motion_interval")?;
        self.config.pointer_motion_interval = value.parse()?;

        Ok(self.config.pointer_motion_interval)
    }

    pub fn get_pointer_modifier(
        &mut self,
    ) -> Result<PointerModifier, ReplyError> {
        let value = self.get_config_property("pointer_modifier")?;
        self.config.pointer_modifier = value.parse()?;

        Ok(self.config.pointer_modifier)
    }

    pub fn get_pointer_action1(&mut self) -> Result<PointerAction, ReplyError> {
        let value = self.get_config_property("pointer_action1")?;
        self.config.pointer_action1 = value.parse()?;

        Ok(self.config.pointer_action1)
    }

    pub fn get_pointer_action2(&mut self) -> Result<PointerAction, ReplyError> {
        let value = self.get_config_property("pointer_action2")?;
        self.config.pointer_action2 = value.parse()?;

        Ok(self.config.pointer_action2)
    }

    pub fn get_pointer_action3(&mut self) -> Result<PointerAction, ReplyError> {
        let value = self.get_config_property("pointer_action3")?;
        self.config.pointer_action3 = value.parse()?;

        Ok(self.config.pointer_action3)
    }

    pub fn get_click_to_focus(&mut self) -> Result<ClickToFocus, ReplyError> {
        let value = self.get_config_property("click_to_focus")?;
        self.config.click_to_focus = value.parse()?;

        Ok(self.config.click_to_focus)
    }

    pub fn get_swallow_first_click(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("swallow_first_click")?;
        self.config.swallow_first_click = value.parse()?;

        Ok(self.config.swallow_first_click)
    }

    pub fn get_focus_follows_pointer(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("focus_follows_pointer")?;
        self.config.focus_follows_pointer = value.parse()?;

        Ok(self.config.focus_follows_pointer)
    }

    pub fn get_pointer_follows_focus(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("pointer_follows_focus")?;
        self.config.pointer_follows_focus = value.parse()?;

        Ok(self.config.pointer_follows_focus)
    }

    pub fn get_pointer_follows_monitor(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("pointer_follows_monitor")?;
        self.config.pointer_follows_monitor = value.parse()?;

        Ok(self.config.pointer_follows_monitor)
    }

    pub fn get_mapping_events_count(&mut self) -> Result<i32, ReplyError> {
        let value = self.get_config_property("mapping_events_count")?;
        self.config.mapping_events_count = value.parse()?;

        Ok(self.config.mapping_events_count)
    }

    pub fn get_ignore_ewmh_focus(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("ignore_ewmh_focus")?;
        self.config.ignore_ewmh_focus = value.parse()?;

        Ok(self.config.ignore_ewmh_focus)
    }

    pub fn get_ignore_ewmh_fullscreen(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("ignore_ewmh_fullscreen")?;
        self.config.ignore_ewmh_fullscreen = value.parse()?;

        Ok(self.config.ignore_ewmh_fullscreen)
    }

    pub fn get_ignore_ewmh_struts(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("ignore_ewmh_struts")?;
        self.config.ignore_ewmh_struts = value.parse()?;

        Ok(self.config.ignore_ewmh_struts)
    }

    pub fn get_center_pseudo_tiled(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("center_pseudo_tiled")?;
        self.config.center_pseudo_tiled = value.parse()?;

        Ok(self.config.center_pseudo_tiled)
    }

    pub fn get_honor_size_hints(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("honor_size_hints")?;
        self.config.honor_size_hints = value.parse()?;

        Ok(self.config.honor_size_hints)
    }

    pub fn get_remove_disabled_monitors(&mut self) -> Result<bool, ReplyError> {
        let value = self.get_config_property("remove_disabled_monitors")?;
        self.config.remove_disabled_monitors = value.parse()?;

        Ok(self.config.remove_disabled_monitors)
    }

    pub fn get_remove_unplugged_monitors(
        &mut self,
    ) -> Result<bool, ReplyError> {
        let value = self.get_config_property("remove_unplugged_monitors")?;
        self.config.remove_unplugged_monitors = value.parse()?;

        Ok(self.config.remove_unplugged_monitors)
    }

    pub fn get_merge_overlapping_monitors(
        &mut self,
    ) -> Result<bool, ReplyError> {
        let value = self.get_config_property("merge_overlapping_monitors")?;
        self.config.merge_overlapping_monitors = value.parse()?;

        Ok(self.config.merge_overlapping_monitors)
    }
}

impl BspwmConnection {
    pub fn set_normal_border_color(
        &mut self,
        value: String,
    ) -> Result<(), ReplyError> {
        self.set_config_property("normal_border_color", &value.to_string())?;
        self.config.normal_border_color = value;

        Ok(())
    }

    pub fn set_active_border_color(
        &mut self,
        value: String,
    ) -> Result<(), ReplyError> {
        self.set_config_property("active_border_color", &value.to_string())?;
        self.config.active_border_color = value;

        Ok(())
    }

    pub fn set_focused_border_color(
        &mut self,
        value: String,
    ) -> Result<(), ReplyError> {
        self.set_config_property("focused_border_color", &value.to_string())?;
        self.config.focused_border_color = value;

        Ok(())
    }

    pub fn set_presel_feedback_color(
        &mut self,
        value: String,
    ) -> Result<(), ReplyError> {
        self.set_config_property("presel_feedback_color", &value.to_string())?;
        self.config.presel_feedback_color = value;

        Ok(())
    }

    pub fn set_split_ratio(&mut self, value: f32) -> Result<(), ReplyError> {
        self.set_config_property("split_ratio", &value.to_string())?;
        self.config.split_ratio = value;

        Ok(())
    }

    pub fn set_status_prefix(
        &mut self,
        value: String,
    ) -> Result<(), ReplyError> {
        self.set_config_property("status_prefix", &value.to_string())?;
        self.config.status_prefix = value;

        Ok(())
    }

    pub fn set_external_rules_command(
        &mut self,
        value: String,
    ) -> Result<(), ReplyError> {
        self.set_config_property("external_rules_command", &value.to_string())?;
        self.config.external_rules_command = value;

        Ok(())
    }

    pub fn set_automatic_scheme(
        &mut self,
        value: Scheme,
    ) -> Result<(), ReplyError> {
        self.set_config_property("automatic_scheme", &value.to_string())?;
        self.config.automatic_scheme = value;

        Ok(())
    }

    pub fn set_initial_polarity(
        &mut self,
        value: Polarity,
    ) -> Result<(), ReplyError> {
        self.set_config_property("initial_polarity", &value.to_string())?;
        self.config.initial_polarity = value;

        Ok(())
    }

    pub fn set_directional_focus_tightness(
        &mut self,
        value: Tightness,
    ) -> Result<(), ReplyError> {
        self.set_config_property(
            "directional_focus_tightness",
            &value.to_string(),
        )?;
        self.config.directional_focus_tightness = value;

        Ok(())
    }

    pub fn set_removal_adjustment(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("removal_adjustment", &value.to_string())?;
        self.config.removal_adjustment = value;

        Ok(())
    }

    pub fn set_presel_feedback(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("presel_feedback", &value.to_string())?;
        self.config.presel_feedback = value;

        Ok(())
    }

    pub fn set_borderless_monocle(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("borderless_monocle", &value.to_string())?;
        self.config.borderless_monocle = value;

        Ok(())
    }

    pub fn set_gapless_monocle(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("gapless_monocle", &value.to_string())?;
        self.config.gapless_monocle = value;

        Ok(())
    }

    pub fn set_top_monocle_padding(
        &mut self,
        value: i16,
    ) -> Result<(), ReplyError> {
        self.set_config_property("top_monocle_padding", &value.to_string())?;
        self.config.top_monocle_padding = value;

        Ok(())
    }

    pub fn set_right_monocle_padding(
        &mut self,
        value: i16,
    ) -> Result<(), ReplyError> {
        self.set_config_property("right_monocle_padding", &value.to_string())?;
        self.config.right_monocle_padding = value;

        Ok(())
    }

    pub fn set_bottom_monocle_padding(
        &mut self,
        value: i16,
    ) -> Result<(), ReplyError> {
        self.set_config_property("bottom_monocle_padding", &value.to_string())?;
        self.config.bottom_monocle_padding = value;

        Ok(())
    }

    pub fn set_left_monocle_padding(
        &mut self,
        value: i16,
    ) -> Result<(), ReplyError> {
        self.set_config_property("left_monocle_padding", &value.to_string())?;
        self.config.left_monocle_padding = value;

        Ok(())
    }

    pub fn set_single_monocle(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("single_monocle", &value.to_string())?;
        self.config.single_monocle = value;

        Ok(())
    }

    pub fn set_pointer_motion_interval(
        &mut self,
        value: u16,
    ) -> Result<(), ReplyError> {
        self.set_config_property(
            "pointer_motion_interval",
            &value.to_string(),
        )?;
        self.config.pointer_motion_interval = value;

        Ok(())
    }

    pub fn set_pointer_modifier(
        &mut self,
        value: PointerModifier,
    ) -> Result<(), ReplyError> {
        self.set_config_property("pointer_modifier", &value.to_string())?;
        self.config.pointer_modifier = value;

        Ok(())
    }

    pub fn set_pointer_action1(
        &mut self,
        value: PointerAction,
    ) -> Result<(), ReplyError> {
        self.set_config_property("pointer_action1", &value.to_string())?;
        self.config.pointer_action1 = value;

        Ok(())
    }

    pub fn set_pointer_action2(
        &mut self,
        value: PointerAction,
    ) -> Result<(), ReplyError> {
        self.set_config_property("pointer_action2", &value.to_string())?;
        self.config.pointer_action2 = value;

        Ok(())
    }

    pub fn set_pointer_action3(
        &mut self,
        value: PointerAction,
    ) -> Result<(), ReplyError> {
        self.set_config_property("pointer_action3", &value.to_string())?;
        self.config.pointer_action3 = value;

        Ok(())
    }

    pub fn set_click_to_focus(
        &mut self,
        value: ClickToFocus,
    ) -> Result<(), ReplyError> {
        self.set_config_property("click_to_focus", &value.to_string())?;
        self.config.click_to_focus = value;

        Ok(())
    }

    pub fn set_swallow_first_click(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("swallow_first_click", &value.to_string())?;
        self.config.swallow_first_click = value;

        Ok(())
    }

    pub fn set_focus_follows_pointer(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("focus_follows_pointer", &value.to_string())?;
        self.config.focus_follows_pointer = value;

        Ok(())
    }

    pub fn set_pointer_follows_focus(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("pointer_follows_focus", &value.to_string())?;
        self.config.pointer_follows_focus = value;

        Ok(())
    }

    pub fn set_pointer_follows_monitor(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property(
            "pointer_follows_monitor",
            &value.to_string(),
        )?;
        self.config.pointer_follows_monitor = value;

        Ok(())
    }

    pub fn set_mapping_events_count(
        &mut self,
        value: i32,
    ) -> Result<(), ReplyError> {
        self.set_config_property("mapping_events_count", &value.to_string())?;
        self.config.mapping_events_count = value;

        Ok(())
    }

    pub fn set_ignore_ewmh_focus(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("ignore_ewmh_focus", &value.to_string())?;
        self.config.ignore_ewmh_focus = value;

        Ok(())
    }

    pub fn set_ignore_ewmh_fullscreen(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("ignore_ewmh_fullscreen", &value.to_string())?;
        self.config.ignore_ewmh_fullscreen = value;

        Ok(())
    }

    pub fn set_ignore_ewmh_struts(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("ignore_ewmh_struts", &value.to_string())?;
        self.config.ignore_ewmh_struts = value;

        Ok(())
    }

    pub fn set_center_pseudo_tiled(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("center_pseudo_tiled", &value.to_string())?;
        self.config.center_pseudo_tiled = value;

        Ok(())
    }

    pub fn set_honor_size_hints(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property("honor_size_hints", &value.to_string())?;
        self.config.honor_size_hints = value;

        Ok(())
    }

    pub fn set_remove_disabled_monitors(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property(
            "remove_disabled_monitors",
            &value.to_string(),
        )?;
        self.config.remove_disabled_monitors = value;

        Ok(())
    }

    pub fn set_remove_unplugged_monitors(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property(
            "remove_unplugged_monitors",
            &value.to_string(),
        )?;
        self.config.remove_unplugged_monitors = value;

        Ok(())
    }

    pub fn set_merge_overlapping_monitors(
        &mut self,
        value: bool,
    ) -> Result<(), ReplyError> {
        self.set_config_property(
            "merge_overlapping_monitors",
            &value.to_string(),
        )?;
        self.config.merge_overlapping_monitors = value;

        Ok(())
    }
}
