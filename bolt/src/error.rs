use std::boxed::Box;
use std::fmt;
use std::io;

use packstream_serde::error::PackstreamError;
use packstream_serde::message::SummaryMessage;

pub type BoltResult<T> = std::result::Result<T, BoltError>;

#[derive(Debug)]
pub struct BoltError {
    err: Box<ErrorCode>,
}

impl BoltError {
    pub fn create(msg: impl Into<ErrorCode>) -> Self {
        Self {
            err: Box::new(msg.into()),
        }
    }
}

impl fmt::Display for BoltError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl std::error::Error for BoltError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        Some(self)
    }
}

impl<T> From<T> for BoltError
where
    T: Into<ErrorCode>,
{
    fn from(value: T) -> Self {
        Self::create(value)
    }
}

#[derive(Debug)]
pub enum ErrorCode {
    IO(io::Error),
    Message(String),
    Packstream(PackstreamError),
    Bolt(SummaryMessage),
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::IO(error) => write!(f, "{}", error),
            Self::Message(string) => write!(f, "{}", string),
            Self::Packstream(error) => write!(f, "{}", error),
            Self::Bolt(message) => write!(f, "{}", message),
        }
    }
}

impl From<String> for ErrorCode {
    fn from(string: String) -> Self {
        Self::Message(string)
    }
}

impl From<&str> for ErrorCode {
    fn from(string: &str) -> Self {
        Self::Message(string.to_owned())
    }
}

impl From<PackstreamError> for ErrorCode {
    fn from(error: PackstreamError) -> Self {
        Self::Packstream(error)
    }
}

impl From<io::Error> for ErrorCode {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<SummaryMessage> for ErrorCode {
    fn from(message: SummaryMessage) -> Self {
        Self::Bolt(message)
    }
}
