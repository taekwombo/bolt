use serde::{de, ser};
use std::fmt;

pub type PackstreamResult<T> = std::result::Result<T, PackstreamError>;

#[derive(Debug)]
pub struct PackstreamError {
    err: Box<ErrorCode>,
}

impl PackstreamError {
    pub(crate) fn create(msg: impl Into<ErrorCode>) -> Self {
        Self {
            err: Box::new(msg.into()),
        }
    }

    pub(crate) fn impl_err(msg: impl Into<String>) -> Self {
        Self {
            err: Box::new(ErrorCode::ImplementationError(msg.into())),
        }
    }
}

impl fmt::Display for PackstreamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl ser::Error for PackstreamError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::create(msg.to_string())
    }
}

impl de::Error for PackstreamError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::create(msg.to_string())
    }

    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        if let de::Unexpected::Unit = unexp {
            PackstreamError::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            PackstreamError::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}

impl std::error::Error for PackstreamError {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<std::str::Utf8Error> for PackstreamError {
    fn from(m: std::str::Utf8Error) -> Self {
        Self::create(m.to_string())
    }
}

impl From<std::string::FromUtf8Error> for PackstreamError {
    fn from(m: std::string::FromUtf8Error) -> Self {
        Self::create(m.to_string())
    }
}

#[derive(Debug)]
pub(crate) enum ErrorCode {
    Message(String),
    ImplementationError(String),
    UnexpectedEndOfBytes,
    UnexpectedTrailingBytes,
    VirtualIllegalAssignment,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(b_str) => write!(f, "{}", b_str),
            Self::ImplementationError(b_str) => write!(
                f,
                "{}\n - This is an implementation error, check custom Serde implementations",
                b_str
            ),
            Self::UnexpectedEndOfBytes => write!(f, "Unexpected end of bytes"),
            Self::UnexpectedTrailingBytes => {
                write!(f, "Unexpected trailing bytes left in the input")
            }
            Self::VirtualIllegalAssignment => write!(
                f,
                "Virtual marker and value must be consumed before setting new one"
            ),
        }
    }
}

impl From<String> for ErrorCode {
    fn from(s: String) -> Self {
        Self::Message(s)
    }
}

impl From<&str> for ErrorCode {
    fn from(s: &str) -> Self {
        Self::Message(s.to_owned())
    }
}
