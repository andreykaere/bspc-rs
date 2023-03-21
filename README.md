# bspc-rs

[![CI](https://github.com/andreykaere/bspc-rs/workflows/CI/badge.svg)](https://github.com/andreykaere/bspc-rs/actions)
[![crate](https://img.shields.io/crates/v/bspc-rs.svg)](https://crates.io/crates/bspc-rs)
[![docs](https://docs.rs/bspc-rs/badge.svg)](https://docs.rs/bspc-rs)

This library provides an implementation of
[`bspc`](https://github.com/baskerville/bspwm) in rust programming language.
Feel free to open any issue or ask any questions.

## Example

```rust, no_run
use bspc_rs as bspc;
use std::error::Error;
use bspc::events::{Subscription, Event, NodeEvent};


fn main() {
    let subscriptions = vec![
        Subscription::NodeAdd,
        Subscription::NodeFocus,
        Subscription::NodeFlag,
        Subscription::NodeState,
        Subscription::NodeRemove,
    ];

    let mut subscriber = bspc::subscribe(&subscriptions, false, None).unwrap();

    for event in subscriber.events() {
        match event.unwrap() {
            Event::NodeEvent(event) => match event {
                NodeEvent::NodeFocus(node_info) => {
                    println!("Window with id {} is focused!", node_info.node_id);
                }
                NodeEvent::NodeRemove(node_info) => {
                    println!("Window with id {} was removed!", node_info.node_id);
                }
                _ => {}
            },
            _ => {}
        }
    }
}
```

