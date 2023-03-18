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

pub(crate) fn connect() -> Result<UnixStream, ReplyError> {
    let socket_path = locate_socket();
    let stream = UnixStream::connect(socket_path)?;

    Ok(stream)
}
