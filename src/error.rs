use serde::{de, ser};
use std::fmt;

pub type SerdeResult<T> = std::result::Result<T, SerdeError>;

#[derive(Debug)]
pub struct SerdeError {
    err: Box<ErrorCode>,
}

impl SerdeError {
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

impl fmt::Display for SerdeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl ser::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::create(msg.to_string())
    }
}

impl de::Error for SerdeError {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::create(msg.to_string())
    }

    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        if let de::Unexpected::Unit = unexp {
            SerdeError::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            SerdeError::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}

impl std::error::Error for SerdeError {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<std::str::Utf8Error> for SerdeError {
    fn from(m: std::str::Utf8Error) -> Self {
        Self::create(m.to_string())
    }
}

impl From<std::string::FromUtf8Error> for SerdeError {
    fn from(m: std::string::FromUtf8Error) -> Self {
        Self::create(m.to_string())
    }
}

#[derive(Debug)]
pub(crate) enum ErrorCode {
    Message(String),
    ImplementationError(String),
    ExpectedSizeMarker,
    ExpectedString1Marker,
    ExpectedStringMarker,
    ExpectedIntMarker,
    ExpectedFloatMarker,
    ExpectedBoolMarker,
    ExpectedBytesMarker,
    UnexpectedType,
    ExpectedListMarker,
    UnexpectedEOSMarker,
    UnexpectedEndOfBytes,
    UnexpectedTrailingBytes,
    VirtualIllegalAssignment,
    VirutalExpectedIntMarker,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(b_str) => write!(f, "{}", b_str),
            Self::ImplementationError(b_str) => write!(
                f,
                "{}\n. This is an implementation error, submit issue at: <address>",
                b_str
            ),
            Self::ExpectedSizeMarker => write!(f, "Expected size."),
            Self::ExpectedString1Marker => write!(f, "Expected String(1) Marker."),
            Self::ExpectedStringMarker => write!(f, "Expected String Marker."),
            Self::ExpectedIntMarker => write!(f, "Expected String Marker."),
            Self::ExpectedFloatMarker => write!(f, "Expected Float Marker."),
            Self::ExpectedBoolMarker => write!(f, "Expected Bool Marker."),
            Self::ExpectedBytesMarker => write!(f, "Expected Bytes Marker."),
            Self::UnexpectedType => write!(f, "Unexpected Type."),
            Self::ExpectedListMarker => write!(f, "Expected List Marker."),
            Self::UnexpectedEndOfBytes => write!(f, "Unexpected end of bytes."),
            Self::UnexpectedTrailingBytes => {
                write!(f, "Unexpected trailing bytes left in the input.")
            }
            Self::UnexpectedEOSMarker => write!(f, "Unexpected End Of Stream marker."),
            Self::VirtualIllegalAssignment => write!(
                f,
                "Virtual marker and value must be consumed before setting new one."
            ),
            Self::VirutalExpectedIntMarker => write!(f, "Virtual"),
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
