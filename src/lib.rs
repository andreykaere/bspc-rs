pub mod errors;
pub mod events;
pub mod properties;
pub mod query;
pub mod selectors;
// pub mod settings;
pub mod tree;

mod parser;
mod socket;

pub type Id = u32;

#[doc(inline)]
pub use crate::events::subscribe;

#[doc(inline)]
pub use crate::query::{
    query_desktops, query_monitors, query_nodes, query_tree,
};

#[doc(inline)]
pub use crate::tree::{
    from_id_to_desktop, from_id_to_monitor, from_id_to_node,
};

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

        let subscribers = subscribe(&subscriptions, false, None).unwrap();

        for event in subscribers {
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
