use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;
use wjp::{ParseError, Serialize, Values};

use crate::error::{HttpParseError, ParseErrorKind::Version};
use crate::util::{error_option_empty, INDEX_WAS_WRONG};

const NAME_NOT_EXIST: &str = "Couldn't find a valid HTTP Version to that string";
const NAMES: [&str; 4] = ["HTTP/1.0", "HTTP/1.1", "HTTP/2", "HTTP/3"];

/// Enum for the 4 different HTTP Version
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,Default)]
pub enum HttpVersion {
    /// HTTP/1.0
    One,
    /// HTTP/1.1
    #[default]
    OnePointOne,
    /// HTTP/2
    Two,
    /// HTTP/3
    Three,
}

impl FromStr for HttpVersion {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAMES
            .iter()
            .position(|&idx| idx.eq_ignore_ascii_case(s))
            .map(HttpVersion::try_from)
            .ok_or(HttpParseError::from((Version, NAME_NOT_EXIST)))?
    }
}

impl TryFrom<Option<&str>> for HttpVersion {
    type Error = HttpParseError;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        value.ok_or(error_option_empty(Version)).map(Self::from_str)?
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
            _ => Err(HttpParseError::from((Version,INDEX_WAS_WRONG))),
        }
    }
}

impl TryFrom<Values> for HttpVersion{
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        HttpVersion::from_str(value.get_string().ok_or(ParseError::new())?.as_str())
            .map_err(|_err|ParseError::new())
    }
}
impl Serialize for HttpVersion{
    fn serialize(&self) -> Values {
        Values::String(self.to_string())
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
            Err(HttpParseError::from(Version))
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
        Debug::fmt(self, f)
    }
}
