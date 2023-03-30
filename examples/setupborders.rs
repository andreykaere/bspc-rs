// This script creates borders around the windows, when there are more, then
// one window on the current desktop

use bspc_rs::events::{self, Event, NodeEvent, Subscription};
use bspc_rs::selectors::NodeSelector;
use bspc_rs::settings;
use bspc_rs::Id;

// Apps names, which you don't want to have border for
const IGNORE_APPS: [&str; 3] = ["TelegramDesktop", "Audacious", "Spotify"];

// Apps names, for which you want to have border
const SET_BORDER: [&str; 4] =
    ["Alacritty", "Gnome-terminal", "Zathura", "Nsxiv"];

// Path to the file, where to dump current state about which windows have apps
// and which don't
const WINDOWS_FILE: &str = "/tmp/bordered_windows";

// Width of the border to set (in pixels)
const BORDER_WIDTH: i32 = 2;

fn main() {
    let subscriptions = [Subscription::NodeAdd, Subscription::NodeRemove];
    let mut subscriber =
        events::subscribe(false, None, &subscriptions).unwrap();

    for event in subscriber.events() {
        match event.unwrap() {
            Event::NodeEvent(event) => match event {
                NodeEvent::NodeAdd(node_info) => {}

                NodeEvent::NodeRemove(node_info) => {}

                _ => unreachable!(),
            },

            _ => unreachable!(),
        }
    }
}

fn create_border(id: Id) {
    settings::set_border_width(
        None,
        None,
        Some(NodeSelector(&id.to_string())),
        BORDER_WIDTH,
    )
    .unwrap();
}
