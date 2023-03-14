use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;

use crate::errors::ReplyError;
use crate::BspwmConnection;

pub trait BspcCommunication {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError>;
    fn receive_message(&mut self) -> Result<String, ReplyError>;

    // fn receive_send_message(&mut self) -> Result<String, ReplyError> {
    //     let reply = self.receive_message()?;
    //     self.send_message(&reply)?;

    //     Ok(reply)
    // }
}

impl BspcCommunication for UnixStream {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
        println!("write start");
        self.write_all(message.as_bytes())?;
        self.flush()?;
        println!("write end");

        Ok(())
    }

    fn receive_message(&mut self) -> Result<String, ReplyError> {
        let mut stream = BufReader::new(self);
        let mut buf = Vec::new();

        stream.read_until(10, &mut buf)?;

        if buf[0] == 7 {
            let reply = String::from_utf8((&buf[1..]).to_vec())?;
            return Err(ReplyError::RequestFailed(reply));
        }

        let reply = String::from_utf8(buf)?;
        Ok(reply)
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
