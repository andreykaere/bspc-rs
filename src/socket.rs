use std::env;
use std::io::Write;
use std::io::{BufRead, BufReader};
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

pub trait BspcCommunication {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError>;
    fn receive_message(&mut self) -> Result<Vec<String>, ReplyError>;
}

impl BspcCommunication for UnixStream {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
        self.write_all(message.as_bytes())?;
        self.flush()?;

        Ok(())
    }

    fn receive_message(&mut self) -> Result<Vec<String>, ReplyError> {
        let buf_reader = BufReader::new(self);
        let mut lines_iter = buf_reader.lines();
        let mut result = Vec::new();

        let first_line = match lines_iter.next() {
            Some(x) => x?,
            None => {
                return Err(ReplyError::NoReply);
            }
        };
        let first_line = first_line.as_bytes();

        if first_line[0] == 7 {
            let reply = String::from_utf8(first_line[1..].to_vec())?;
            return Err(ReplyError::RequestFailed(reply));
        } else {
            let first_line = String::from_utf8(first_line.to_vec())?;
            result.push(first_line);
        }

        for line in lines_iter {
            result.push(line?);
        }

        Ok(result)
    }
}
