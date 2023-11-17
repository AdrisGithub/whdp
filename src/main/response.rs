use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::error::{HttpParseError, ParseErrorKind};
use crate::status::HttpStatus;
use crate::status::presets::ok;
use crate::util::{Destruct, EMPTY_CHAR, parse_body, parse_header, ParseKeyValue};
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
    pub fn set_body(&mut self, body: String) -> &mut Response {
        self.body = body;
        self
    }
    pub fn set_version(&mut self, version: HttpVersion) -> &mut Response {
        self.version = version;
        self
    }
    pub fn set_status(&mut self, status: HttpStatus) -> &mut Response {
        self.status = status;
        self
    }
    pub fn add_header(&mut self, kv: (String, String)) -> &mut Response {
        self.headers.insert(kv.0, kv.1);
        self
    }
    pub fn remove_header(&mut self, key: &String) -> &mut Response {
        self.headers.remove(key);
        self
    }
    pub fn get_headers_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.headers
    }
    pub fn append_body_str(&mut self, str: &str) -> &mut Response {
        self.body.push_str(str);
        self
    }
    pub fn append_body(&mut self, str: String) -> &mut Response {
        self.append_body_str(str.as_str());
        self
    }
    fn parse_meta_line(str: Option<&str>) -> Result<(HttpVersion, HttpStatus), HttpParseError> {
        let mut split = str.ok_or(HttpParseError::from(ParseErrorKind::Req))?
            .split(EMPTY_CHAR);
        let version = HttpVersion::try_from(split.next())?;
        let status = HttpStatus::try_from((
            split.next().ok_or(HttpParseError::from(ParseErrorKind::Req))?,
            split.next().ok_or(HttpParseError::from(ParseErrorKind::Req))?,
        ))?;
        Ok((version, status))
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

impl Debug for Response {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
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

impl Default for Response {
    fn default() -> Self {
        Self {
            headers: BTreeMap::new(),
            status: ok(),
            version: HttpVersion::OnePointOne,
            body: String::from("Hello, World"),
        }
    }
}

impl Destruct for Response {
    type Item = (HttpVersion, HttpStatus, BTreeMap<String, String>, String);
    fn destruct(self) -> Self::Item {
        (self.version, self.status, self.headers, self.body)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use crate::response::Response;

    #[test]
    fn test() {
        let string = read_to_string("src/resources/response.txt").unwrap();
        let resp = Response::try_from(string).unwrap();
        println!("{:?}", resp);
    }
}
