use std::error::Error;

mod events;

pub enum Subscription {
    All,
    Report,
    Monitor,
    Desktop,
    Node,
    MonitorAdd,
    MonitorRename,
    MonitorSwap,
    MonitorFocus,
    MonitorGeometry,
    DesktopAdd,
    DesktopRename,
    DesktopRemove,
    DesktopSwap,
    DesktopTransfer,
    DesktopFocus,
    DesktopActivate,
    DesktopLayout,
    NodeAdd,
    NodeRemove,
    NodeSwap,
    NodeTransfer,
    NodeFocus,
    NodeActivate,
    NodePresel,
    NodeStack,
    NodeGeometry,
    NodeState,
    NodeFlag,
    NodeLayer,
}

pub fn subscribe(
    fifo: Option<()>,
    count: Option<u32>,
    subscriptions: &[Subscription],
) -> Result<(), Box<dyn Error>> {
    todo!();
}

#[cfg(test)]
mod test {
    use std::error::Error;
    use std::io::{self, Read, Write};
    use std::os::unix::net::UnixStream;

    #[test]
    fn test_subscribe() -> Result<(), Box<dyn Error>> {
        let mut stream = UnixStream::connect("/tmp/bspwm_0_0-socket")?;
        let mut buf = [0u8; 1024];

        stream.write_all(b"subscribe\x00node_remove\x00")?;

        loop {
            let len = stream.read(&mut buf)?;
            if len == 0 {
                break;
            }
            let response = String::from_utf8_lossy(&buf[..len]);
            println!("{}", response);
        }

        Ok(())
    }
}
