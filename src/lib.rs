use std::env;
use std::io;
use std::os::unix::net::UnixStream;

pub mod common;
pub mod events;
mod parser;
pub mod tree;

use events::Subscription;
use tree::Tree;

pub struct BspwmConnection {
    stream: UnixStream,
}

pub struct EventIterator<'a> {
    stream: &'a mut UnixStream,
}

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

pub enum QueryOptions {
    Monitor,
    Desktop,
    Node,
}

pub enum QuerySelectors {
    // Monitor(MonitorSelector),
    // Desktop(DesktopSelector),
    // Node(NodeSelector),
}

impl BspwmConnection {
    pub fn connect() -> io::Result<BspwmConnection> {
        let socket_path = locate_socket();
        let stream = UnixStream::connect(socket_path)?;

        Ok(Self { stream })
    }

    pub fn subscribe(
        &mut self,
        fifo: Option<()>,
        count: Option<u32>,
        subscriptions: &[Subscription],
    ) -> io::Result<()> {
        todo!();
    }

    pub fn listen(&mut self) -> EventIterator {
        todo!();
    }

    pub fn query_nodes(&mut self) -> Vec<i32> {
        todo!();
    }

    pub fn query_desktops(&mut self) -> Vec<i32> {
        todo!();
    }

    pub fn query_monitors(&mut self) -> Vec<i32> {
        todo!();
    }

    pub fn query_tree(
        &mut self,
        opts: QueryOptions,
        sel: QuerySelectors,
    ) -> Tree {
        todo!();
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
    fn test_subscribe() -> Result<(), Box<dyn Error>> {
        let mut stream = UnixStream::connect("/tmp/bspwm_0_0-socket")?;
        let mut buf = [0u8; 1024];

        stream.write_all(b"subscribe\x00node\x00")?;

        loop {
            let len = stream.read(&mut buf)?;
            if len == 0 {
                break;
            }
            let response = String::from_utf8_lossy(&buf[..len]);

            // println!("{response}");
            // let reply = response.parse::<NodeEvent>();

            for res in response.split('\n') {
                if res.is_empty() {
                    continue;
                }

                let reply = res.parse::<NodeEvent>()?;

                println!("{reply:#?}");
            }
        }

        Ok(())
    }
}
