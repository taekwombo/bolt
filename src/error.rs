use serde::{ser, de};
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    err: Box<ErrorCode>
}

impl Error {
    pub(crate) fn make(msg: impl Into<ErrorCode>) -> Self {
        Self {
            err: Box::new(msg.into())
        }
    }

    pub(crate) fn from_code(code: ErrorCode) -> Self {
        Self {
            err: Box::new(code)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.err)
    }
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::make(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Self::make(msg.to_string())
    }

    fn invalid_type(unexp: de::Unexpected, exp: &dyn de::Expected) -> Self {
        if let de::Unexpected::Unit = unexp {
            Error::custom(format_args!("invalid type: null, expected {}", exp))
        } else {
            Error::custom(format_args!("invalid type: {}, expected {}", unexp, exp))
        }
    }
}

impl std::error::Error for Error {
    fn cause(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

#[derive(Debug)]
pub(crate) enum ErrorCode {
    Message(Box<str>),
    ExpectedMarkerByte,     // IMPLEMENTATION ERROR. Not a byte that is a marker
    ExpectedSizeMarker,     // IMPLEMENTATION ERROR. Marker that stores size
    MarkerSizeOutOfRange,   // IMPLEMENTATION ERROR. Size overflowed?
    ExpectedString1Marker,
    ExpectedStringMarker,
    ExpectedIntMarker,
    ExpectedFloatMarker,
    ExpectedBoolMarker,
    ExpectedBytesMarker,
    UnexpectedType,
    ExpectedListMarker,
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Message(b_str) => write!(f, "{}", b_str),
            Self::ExpectedSizeMarker => write!(f, "Expected size."),
            Self::ExpectedMarkerByte => write!(f, "Attempt to convert arbitrary u8 into Marker."),
            Self::MarkerSizeOutOfRange => write!(f, "Attempt to create Marker with size higher than maximum allowed value."),
            Self::ExpectedString1Marker => write!(f, "Expected String(1) Marker."),
            Self::ExpectedStringMarker => write!(f, "Expected String Marker."),
            Self::ExpectedIntMarker => write!(f, "Expected String Marker."),
            Self::ExpectedFloatMarker => write!(f, "Expected Float Marker."),
            Self::ExpectedBoolMarker => write!(f, "Expected Bool Marker."),
            Self::ExpectedBytesMarker => write!(f, "Expected Bytes Marker."),
            Self::UnexpectedType => write!(f, "Unexpected Type."),
            Self::ExpectedListMarker => write!(f, "Expected List Marker."),
        }
    }
}

impl From<String> for ErrorCode {
    fn from(s: String) -> Self {
        Self::Message(s.into_boxed_str())
    }
}

impl From<&str> for ErrorCode {
    fn from(s: &str) -> Self {
        Self::Message(s.to_owned().into_boxed_str())
    }
}
