use core::fmt;
use std::error::Error;
use std::io;
use std::os::unix::net::UnixStream;
use strum;

pub use crate::parser::errors::*;

#[derive(Debug)]
pub enum ReplyError {
    ConnectionError(io::Error),
    ParseError(ParseError),
}

impl Error for ReplyError {}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplyError::ConnectionError(err) => err.fmt(f),
            ReplyError::ParseError(err) => err.fmt(f),
        }
    }
}

impl std::convert::From<io::Error> for ReplyError {
    fn from(error: io::Error) -> ReplyError {
        ReplyError::ConnectionError(error)
    }
}

impl<T: std::convert::Into<ParseError>> std::convert::From<T> for ReplyError {
    fn from(error: T) -> ReplyError {
        ReplyError::ParseError(Into::into(error))
    }
}
