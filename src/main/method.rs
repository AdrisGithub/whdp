use std::str::FromStr;

use crate::error::HttpParseError;

const NAMES: [&str; 10] = ["POST", "GET", "PUT", "UPDATE", "DELETE", "PATCH", "HEAD", "CONNECT", "OPTIONS", "TRACE"];

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum HttpMethod {
    Post,
    Get,
    Put,
    Update,
    Delete,
    Patch,
    Head,
    Connect,
    Options,
    Trace,
}

impl FromStr for HttpMethod {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        NAMES.iter()
            .position(|&idx| idx.eq_ignore_ascii_case(s))
            .map(|x| HttpMethod::try_from(x).unwrap())
            .ok_or(HttpParseError::new())
    }
}

impl TryFrom<usize> for HttpMethod {
    type Error = HttpParseError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(HttpMethod::Post),
            1 => Ok(HttpMethod::Get),
            2 => Ok(HttpMethod::Put),
            3 => Ok(HttpMethod::Update),
            4 => Ok(HttpMethod::Delete),
            5 => Ok(HttpMethod::Patch),
            6 => Ok(HttpMethod::Head),
            7 => Ok(HttpMethod::Connect),
            8 => Ok(HttpMethod::Options),
            9 => Ok(HttpMethod::Trace),
            _ => Err(HttpParseError::new())
        }
    }
}