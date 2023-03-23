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
use bspc::events::{self, Subscription, Event, NodeEvent};


fn main() {
    let subscriptions = vec![
        Subscription::NodeAdd,
        Subscription::NodeFocus,
        Subscription::NodeFlag,
        Subscription::NodeState,
        Subscription::NodeRemove,
    ];

    let mut subscriber = events::subscribe(&subscriptions, false, None).unwrap();

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

## Why choose `bspc-rs`?

- Speed. If your script is dealing with the windows and you want to improve
  the speed, than Rust and `bspc-rs` might be a good choice for you. 

- Static types and compilation. This is another reason to choose `bspc-rs`, 
  because you might catch some errors at compile-time. This is especially true
  for big scripts, when mistake of making silly mistake is more high.

- Native integration in Rust code. For example, if you need to do something,
  besides just dealing with `bspwm`, than you can take advantage of other Rust
  libraries for your needs, that are not extracted into binary programs and
  therefore can't be used in bash scripts.


## More examples

This library was started as a desire to have a native `bspc` support in Rust
in [`ixwindow`](https://github.com/andreykaere/ixwindow) project. 

Some small useful scripts/utilities that I use daily, written with help of
this library, (basically ported from bash) can be found in `examples` directory.

