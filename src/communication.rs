use crate::errors::ReplyError;
use crate::BspwmConnection;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

pub trait BspcCommunication {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError>;
    fn receive_message(&mut self) -> Result<String, ReplyError>;
}

impl BspcCommunication for UnixStream {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
        self.write_all(message.as_bytes())?;
        Ok(())
    }

    // TODO: add proper reply handling and error printing/conversion
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
