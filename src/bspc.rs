use std::env;
use std::os::unix::net::UnixStream;

use crate::errors::ReplyError;

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
    let socket_path = locate_socket();
    let stream = UnixStream::connect(socket_path)?;

    Ok(stream)
}

#[cfg(test)]
mod test {
    use super::events::*;
    use super::*;

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

        let mut subscribers =
            events::subscribe(&subscriptions, false, None).unwrap();

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
