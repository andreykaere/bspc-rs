use std::collections::VecDeque;
use std::env;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

pub mod communication;
pub mod errors;
pub mod events;
mod parser;
pub mod properties;
pub mod query;
pub mod selectors;
pub mod settings;
pub mod tree;

pub use errors::{ParseError, ReplyError};
pub use events::{Event, EventIterator, Subscription};

pub type Id = u32;

pub struct BspwmConnection {
    stream: UnixStream,
}

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

    pub fn connect() -> Result<BspwmConnection, ReplyError> {
        let socket_path = Self::locate_socket();
        let stream = UnixStream::connect(socket_path)?;

        Ok(Self { stream })
    }
}

#[cfg(test)]
mod test {
    use super::events::*;
    use super::*;

    #[test]
    fn test_subscribe() {
        let mut conn = BspwmConnection::connect().unwrap();
        let subscriptions =
            vec![Subscription::All, Subscription::MonitorGeometry];
        conn.subscribe(&subscriptions, false, None);
    }

    #[test]
    fn test_iterator() {
        let mut conn = BspwmConnection::connect().unwrap();
        // let subscriptions = vec![Subscription::Desktop];
        let subscriptions = vec![Subscription::Desktop, Subscription::Node];
        conn.subscribe(&subscriptions, false, None);

        for event in conn.listen() {
            println!("{event:#?}");
        }
    }
    // #[test]
    // fn test_subscribe() -> Result<(), Box<dyn Error>> {
    //     let mut stream = UnixStream::connect("/tmp/bspwm_0_0-socket")?;
    //     let mut buf = [0u8; 1024];

    //     stream.write_all(b"subscribe\x00node\x00")?;

    //     loop {
    //         let len = stream.read(&mut buf)?;

    //         println!("{len}");
    //         if len == 0 {
    //             break;
    //         }

    //         let response = String::from_utf8_lossy(&buf[..len]);

    //         // println!("{response}");
    //         // let reply = response.parse::<NodeEvent>();

    //         for res in response.split('\n') {
    //             if res.is_empty() {
    //                 continue;
    //             }

    //             let inn_reply = res.parse::<NodeEvent>()?;

    //             println!("{inn_reply:#?}");
    //         }
    //     }

    //     Ok(())
    // }
}
