use core::fmt;
use std::error::Error;
use std::io;
use std::os::unix::net::UnixStream;
use strum;

pub use crate::parser::errors::*;

#[derive(Debug)]
#[non_exhaustive]
pub enum ReplyError {
    ConnectionError(io::Error),
    ParseError(ParseError),
    InvalidRequest,
}

impl Error for ReplyError {}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplyError::ConnectionError(err) => err.fmt(f),
            ReplyError::ParseError(err) => err.fmt(f),
            ReplyError::InvalidRequest => write!(f, "Given request is invalid"),
        }
    }
}

impl From<io::Error> for ReplyError {
    fn from(error: io::Error) -> ReplyError {
        ReplyError::ConnectionError(error)
    }
}

impl<T: Into<ParseError>> From<T> for ReplyError {
    fn from(error: T) -> ReplyError {
        ReplyError::ParseError(Into::into(error))
    }
}
