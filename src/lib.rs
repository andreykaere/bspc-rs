/*!

This library provides an implementation of
[`bspc`](https://github.com/baskerville/bspwm) in rust programming language.

## Example

```rust, no_run
# #[allow(clippy::needless_doctest_main)]
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

    let mut subscriber = events::subscribe(false, None, &subscriptions).unwrap();

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

*/

#[cfg(doctest)]
#[macro_use]
extern crate doc_comment;

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

pub mod errors;
pub mod events;
pub mod properties;
pub mod query;
pub mod selectors;
pub mod settings;
pub mod tree;

mod parser;
mod socket;

pub type Id = u32;

// #[doc(inline)]
// pub use crate::events::subscribe;

// #[doc(inline)]
// pub use crate::query::{
//     query_desktops, query_monitors, query_nodes, query_tree,
// };

// #[doc(inline)]
// pub use crate::tree::{
//     from_id_to_desktop, from_id_to_monitor, from_id_to_node,
// };

#[cfg(test)]
mod test {
    use super::*;
    use crate::events::*;

    // #[test]
    // fn test_subscribe() {
    //     let subscriptions =
    //         vec![Subscription::All, Subscription::MonitorGeometry];
    //     Bspc::subscribe(&subscriptions, false, None).unwrap();
    // }

    #[test]
    #[ignore]
    fn test_iterator() {
        let subscriptions = vec![
            Subscription::NodeAdd,
            Subscription::NodeFocus,
            Subscription::NodeFlag,
            Subscription::NodeState,
            Subscription::NodeRemove,
        ];

        let mut subscribers = subscribe(false, None, &subscriptions).unwrap();

        for event in subscribers.events() {
            match event.unwrap() {
                Event::NodeEvent(event) => match event {
                    NodeEvent::NodeFocus(_) => {
                        println!("focus!");
                    }
                    NodeEvent::NodeFlag(_) => {
                        println!("flag!");
                    }
                    NodeEvent::NodeRemove(_) => break,
                    _ => {}
                },
                _ => {}
            }
        }

        for event in subscribers.events() {
            match event.unwrap() {
                Event::NodeEvent(event) => match event {
                    NodeEvent::NodeFocus(_) => {
                        println!("focus!");
                    }
                    NodeEvent::NodeFlag(_) => {
                        println!("flag!");
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
}
