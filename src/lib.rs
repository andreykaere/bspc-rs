use std::collections::VecDeque;
use std::env;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

pub mod config;
pub mod errors;
pub mod events;
mod parser;
pub mod properties;
pub mod query;

use config::Config;
use errors::{ParseError, ReplyError};
use events::{Event, EventIterator, Subscription};

pub struct BspwmConnection {
    stream: UnixStream,
    config: Config,
}

pub trait BspcCommunication {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError>;
    fn receive_message(&mut self) -> Result<String, ReplyError>;
}

impl BspcCommunication for UnixStream {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
        self.write_all(message.as_bytes())?;
        Ok(())
    }

    fn receive_message(&mut self) -> Result<String, ReplyError> {
        // https://unix.stackexchange.com/questions/424380/what-values-may-linux-use-for-the-default-unix-socket-buffer-size
        let mut buf = [0u8; 212992];
        let len = self.read(&mut buf)?;

        if len == 0 {
            return Ok(String::new());
        }

        let reply = String::from_utf8_lossy(&buf[..len]);

        Ok(reply.to_string())
    }
}

impl BspcCommunication for BspwmConnection {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
        self.stream.send_message(message)
    }
    fn receive_message(&mut self) -> Result<String, ReplyError> {
        self.stream.receive_message()
    }
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
        let mut stream = UnixStream::connect(socket_path)?;
        let config = Config::load(&mut stream)?;

        Ok(Self { stream, config })
    }

    /// Subscribes to the given events
    // TODO: implement fifo and count, right now have no idea what they do
    pub fn subscribe(
        &mut self,
        fifo: Option<()>,
        count: Option<u32>,
        subscriptions: &[Subscription],
    ) -> Result<(), ReplyError> {
        let all_subscriptions = &subscriptions
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("\x00");

        let mut count_option = String::new();
        let mut fifo_option = "";

        if let Some(x) = count {
            count_option = format!("--count\x00{}\x00", x);
        }

        if let Some(()) = fifo {
            fifo_option = "--fifo\x00";
        }
        let subscribe_message = format!(
            "subscribe\x00{}{}{}\x00",
            fifo_option, count_option, all_subscriptions
        );

        self.stream.send_message(&subscribe_message)
    }

    /// Listen to the subscriptions
    pub fn listen(&mut self) -> EventIterator {
        EventIterator {
            stream: &mut self.stream,
            cache: VecDeque::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::events::*;
    use super::*;
    use std::error::Error;
    use std::io::{self, Read, Write};
    use std::os::unix::net::UnixStream;

    #[test]
    fn test_subscribe() {
        let mut conn = BspwmConnection::connect().unwrap();
        let subscriptions =
            vec![Subscription::All, Subscription::MonitorGeometry];
        conn.subscribe(None, None, &subscriptions);
    }

    #[test]
    fn test_iterator() {
        let mut conn = BspwmConnection::connect().unwrap();
        // let subscriptions = vec![Subscription::Desktop];
        let subscriptions = vec![Subscription::Desktop, Subscription::Node];
        conn.subscribe(None, None, &subscriptions);

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
