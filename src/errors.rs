use core::fmt;
use std::error::Error;
use std::io;
use std::os::unix::net::UnixStream;

pub use super::parser::errors::*;

#[derive(Debug)]
pub enum ReplyError {
    ConnectionError(io::Error),
    ParseError(ParseError),
}

impl Error for ReplyError {}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl std::convert::From<io::Error> for ReplyError {
    fn from(error: io::Error) -> ReplyError {
        ReplyError::ConnectionError(error)
    }
}

impl std::convert::From<ParseError> for ReplyError {
    fn from(error: ParseError) -> ReplyError {
        ReplyError::ParseError(error)
    }
}
