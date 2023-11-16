use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::error::{HttpParseError, ParseErrorKind};

const NAMES: [&str; 4] = ["HTTP/1.0", "HTTP/1.1", "HTTP/2", "HTTP/3"];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HttpVersion {
    One,
    OnePointOne,
    Two,
    Three,
}

impl FromStr for HttpVersion {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAMES.iter()
            .position(|&idx| idx.eq_ignore_ascii_case(s))
            .map(HttpVersion::try_from)
            .ok_or(HttpParseError::from(ParseErrorKind::Version))?
    }
}

impl TryFrom<f32> for HttpVersion {
    type Error = HttpParseError;
    fn try_from(value: f32) -> Result<Self, Self::Error> {
        HttpVersion::try_from(value as f64)
    }
}

impl TryFrom<usize> for HttpVersion {
    type Error = HttpParseError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HttpVersion::One),
            1 => Ok(HttpVersion::OnePointOne),
            2 => Ok(HttpVersion::Two),
            3 => Ok(HttpVersion::Three),
            _ => Err(HttpParseError::from(ParseErrorKind::Version)),
        }
    }
}

impl TryFrom<f64> for HttpVersion {
    type Error = HttpParseError;
    fn try_from(value: f64) -> Result<Self, Self::Error> {
        // pattern matching doesn't work with floating point numbers
        // https://github.com/rust-lang/rust/issues/41255
        if value == 1.0 {
            Ok(HttpVersion::One)
        } else if value == 1.1 {
            Ok(HttpVersion::OnePointOne)
        } else if value == 2.0 {
            Ok(HttpVersion::Two)
        } else if value == 3.0 {
            Ok(HttpVersion::Three)
        } else {
            Err(HttpParseError::from(ParseErrorKind::Version))
        }
    }
}

impl Debug for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAMES[*self as usize])
    }
}

impl Display for HttpVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(f, self)
    }
}