use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::error::{HttpParseError, ParseErrorKind::Method};

const NAMES: [&str; 9] = [
    "POST", "GET", "PUT", "DELETE", "PATCH", "HEAD", "CONNECT", "OPTIONS", "TRACE",
];

/// Enum for all the HTTP Methods
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HttpMethod {
    /// Is used for creating/posting data to a Server. <br>
    /// The difference to [PUT] is that it does not have to be idempotent
    ///
    /// [PUT]: crate::HttpMethod::Put
    Post,
    /// Is used for requesting data from a Server. <br>
    /// It should be Idempotent
    Get,
    /// Is used for replacing data on a Server. <br>
    /// The difference to [POST] is that it should be idempotent
    ///
    /// [POST]: crate::HttpMethod::Post
    Put,
    /// Is used for "deleting" data from a Server <br>
    /// It should be Idempotent
    Delete,
    /// Is used for applying partial modification to data on a Server <br>
    /// It may not be Idempotent
    Patch,
    /// Is used for requesting only the header that would be
    /// returned if a [GET] would have been sent to a Server <br>
    /// It should be Idempotent
    ///
    /// [GET]: crate::HttpMethod::Get
    Head,
    /// Is used for opening a two-way communication with a Server
    /// for example opening a tunnel <br>
    /// It should be Idempotent
    Connect,
    /// Is used for requesting permitted communication options <br>
    /// It should be Idempotent
    Options,
    /// Is used for performing a message loop-back along the path to the requested resource <br>
    /// It's often used for debugging and is Idempotent
    Trace,
}

impl FromStr for HttpMethod {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAMES
            .iter()
            .position(|&idx| idx.eq_ignore_ascii_case(s))
            .map(HttpMethod::try_from)
            .ok_or(HttpParseError::from(Method))?
    }
}

impl TryFrom<Option<&str>> for HttpMethod {
    type Error = HttpParseError;
    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        value.ok_or(HttpParseError::from(Method))
            .map(HttpMethod::from_str)?
    }
}

impl TryFrom<usize> for HttpMethod {
    type Error = HttpParseError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HttpMethod::Post),
            1 => Ok(HttpMethod::Get),
            2 => Ok(HttpMethod::Put),
            3 => Ok(HttpMethod::Delete),
            4 => Ok(HttpMethod::Patch),
            5 => Ok(HttpMethod::Head),
            6 => Ok(HttpMethod::Connect),
            7 => Ok(HttpMethod::Options),
            8 => Ok(HttpMethod::Trace),
            _ => Err(HttpParseError::from(Method)),
        }
    }
}

impl Debug for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", NAMES[*self as usize])
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
