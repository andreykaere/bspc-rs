use std::io::{self, Read, Write};
use std::io::{BufRead, BufReader};
use std::os::unix::net::UnixStream;

use crate::errors::ReplyError;

pub trait BspcCommunication {
    fn send_message(&mut self, message: &str) -> Result<(), ReplyError>;
    fn receive_message(&mut self) -> Result<Vec<String>, ReplyError>;

    // fn receive_send_message(&mut self) -> Result<String, ReplyError> {
    //     let reply = self.receive_message()?;
    //     self.send_message(&reply)?;

    //     Ok(reply)
    // }
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

        // let mut buf = Vec::new();
        // self.read_to_end(&mut buf)?;

        // if buf[0] == 7 {
        //     let reply = String::from_utf8((&buf[1..]).to_vec())?;
        //     return Err(ReplyError::RequestFailed(reply));
        // }

        // let reply = String::from_utf8(buf)?;
        // Ok(reply)
    }

    // fn receive_message(&mut self) -> Result<String, ReplyError> {
    //     let mut stream = BufReader::new(self);
    //     let mut buf = Vec::new();

    //     for line in stream.lines() {
    //         if line[0] == 7 {
    //             return;
    //         }
    //     }
    // stream.read_until(10, &mut buf)?;

    // if buf[0] == 7 {
    //     let reply = String::from_utf8((&buf[1..]).to_vec())?;
    //     return Err(ReplyError::RequestFailed(reply));
    // }

    // let reply = String::from_utf8(buf)?;
    // Ok(reply)
    // }
}

// impl BspcCommunication for Bspc {
//     fn send_message(&mut self, message: &str) -> Result<(), ReplyError> {
//         self.stream.send_message(message)
//     }
//     fn receive_message(&mut self) -> Result<String, ReplyError> {
//         self.stream.receive_message()
//     }
// }
