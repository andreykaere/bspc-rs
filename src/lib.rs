use std::env;
use std::os::unix::net::UnixStream;

pub mod communication;
pub mod errors;
pub mod events;
mod parser;
pub mod properties;
pub mod query;
// pub mod selectors;
// pub mod settings;
pub mod tree;

use errors::{ParseError, ReplyError};
use events::{Event, EventIterator, Subscription};

pub type Id = u32;

// pub struct BspwmConnection {
//     stream: UnixStream,
// }

// pub struct BspwmConnection {
//     stream: UnixStream,
//     buffer: Vec<u8>,
// }

pub struct BspwmConnection;

impl BspwmConnection {
    fn locate_socket() -> String {
        if let Ok(path) = env::var("BSPWM_SOCKET") {
            path
        } else {
            // Examination of the source code has shown that despite man page
            // saying that socket path depends on DISPLAY or other parameters, in
            // fact it always initializing it as presented below
            "/tmp/bspwm_0_0-socket".to_string()
        }
    }

    pub fn connect() -> Result<UnixStream, ReplyError> {
        let socket_path = Self::locate_socket();
        let stream = UnixStream::connect(socket_path)?;
        // let buffer = BufReader::new(stream);

        // Ok(Self { stream, buffer })
        Ok(stream)
    }

    // pub fn connect() -> Result<BspwmConnection, ReplyError> {
    //     let socket_path = Self::locate_socket();
    //     let stream = UnixStream::connect(socket_path)?;
    //     let buffer = Vec::new();
    //     // let buffer = BufReader::new(stream);

    //     Ok(Self { stream, buffer })
    // }

    pub fn new() -> BspwmConnection {
        BspwmConnection
    }
}

#[cfg(test)]
mod test {
    use super::events::*;
    use super::*;

    // #[test]
    // fn test_subscribe() {
    //     let subscriptions =
    //         vec![Subscription::All, Subscription::MonitorGeometry];
    //     BspwmConnection::subscribe(&subscriptions, false, None).unwrap();
    // }

    #[test]
    #[ignore]
    fn test_iterator() {
        let conn = BspwmConnection::new();

        let subscriptions = vec![
            Subscription::NodeAdd,
            Subscription::NodeFocus,
            Subscription::NodeFlag,
            Subscription::NodeState,
            Subscription::NodeRemove,
        ];

        let mut subscribers =
            conn.subscribe(&subscriptions, false, None).unwrap();

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
