//! Contains errors, that are used in this crate.
//!
//! It also has different conversions between differenet types of errors.

use core::fmt;
use std::error::Error;
use std::io;

pub use crate::parser::errors::*;

#[derive(Debug)]
#[non_exhaustive]
pub enum QueryError {
    NoMatches,
    InvalidRequest(String),
    InvalidSelector(String),
}

impl Error for QueryError {}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryError::NoMatches => {
                write!(f, "Query request hasn't have any matches")
            }
            QueryError::InvalidRequest(err) => write!(f, "{}", err),
            QueryError::InvalidSelector(err) => write!(f, "{}", err),
        }
    }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ReplyError {
    ConnectionError(io::Error),
    ParseError(ParseError),
    QueryError(QueryError),
    InvalidRequest(String),
    RequestFailed(String),
    NoReply,
    InvalidSelector(String),
}

impl Error for ReplyError {}

impl fmt::Display for ReplyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ReplyError::ConnectionError(err) => err.fmt(f),
            ReplyError::ParseError(err) => err.fmt(f),
            ReplyError::InvalidRequest(err) => write!(f, "{}", err),
            ReplyError::QueryError(err) => err.fmt(f),
            ReplyError::RequestFailed(err) => write!(f, "{}", err),
            ReplyError::NoReply => {
                write!(f, "No reply was returned to given request")
            }
            ReplyError::InvalidSelector(err) => write!(f, "{}", err),
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
