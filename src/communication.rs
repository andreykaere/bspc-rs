use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;

use crate::errors::ReplyError;

pub trait BspcCommunication {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError>;
    fn receive_message(&mut self) -> Result<String, ReplyError>;
}

impl BspcCommunication for UnixStream {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
        self.write_all(message.as_bytes())?;
        self.flush()?;

        Ok(())
    }

    fn receive_message(&mut self) -> Result<String, ReplyError> {
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;

        if buf[0] == 7 {
            let reply = String::from_utf8((&buf[1..]).to_vec())?;
            return Err(ReplyError::RequestFailed(reply));
        }

        let reply = String::from_utf8(buf)?;
        Ok(reply)
    }
}
