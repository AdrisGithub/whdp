use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

use crate::error::HttpParseError;
use crate::status::HttpStatus;
use crate::util::{parse_body, parse_header, ParseKeyValue, EMPTY_CHAR};
use crate::version::HttpVersion;

pub struct Response {
    version: HttpVersion,
    status: HttpStatus,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Response {
    pub const fn get_version(&self) -> &HttpVersion {
        &self.version
    }
    pub const fn get_headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }
    pub const fn get_status(&self) -> &HttpStatus {
        &self.status
    }
    pub const fn get_body(&self) -> &String {
        &self.body
    }
    pub fn destruct(self) -> (HttpVersion, HttpStatus, BTreeMap<String, String>, String) {
        (self.version, self.status, self.headers, self.body)
    }
    fn parse_meta_line(str: Option<&str>) -> Result<(HttpVersion, HttpStatus), HttpParseError> {
        let mut split = str.ok_or(HttpParseError::new())?.split(EMPTY_CHAR);
        let version = Self::parse_version(split.next())?;
        let status = HttpStatus::try_from((
            split.next().ok_or(HttpParseError::new())?,
            split.next().ok_or(HttpParseError::new())?,
        ))?;
        Ok((version, status))
    }
    fn parse_version(str: Option<&str>) -> Result<HttpVersion, HttpParseError> {
        HttpVersion::from_str(str.ok_or(HttpParseError::new())?)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}\n{}\n{}",
            self.version,
            self.status,
            self.headers.parse_key_value(),
            self.body
        )
    }
}

impl TryFrom<String> for Response {
    type Error = HttpParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}
impl FromStr for Response {
    type Err = HttpParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut value = s.lines();
        let (version, status) = Self::parse_meta_line(value.next())?;
        let headers = parse_header(&mut value)?;
        let body = parse_body(&mut value);
        Ok(Self {
            version,
            status,
            headers,
            body,
        })
    }
}
