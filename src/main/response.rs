use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::error::{HttpParseError, ParseErrorKind::Req};
use crate::status::HttpStatus;
use crate::status::presets::ok;
use crate::util::{Destruct, EMPTY_CHAR, error_option_empty, parse_body, parse_header, ParseKeyValue};
use crate::version::HttpVersion;

const VALIDATE: &str = "min. 1 field was not filled with a value";

/// Struct for representing a HTTP Response
pub struct Response {
    version: HttpVersion,
    status: HttpStatus,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Response {
    /// Creates a new Instance of a [ResponseBuilder]
    /// to "construct" a Response
    pub const fn builder() -> ResponseBuilder {
        ResponseBuilder::new()
    }
    /// Get the [HttpVersion] of your Response
    pub const fn get_version(&self) -> &HttpVersion {
        &self.version
    }
    /// Get the Headers of your Response
    pub const fn get_headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }
    /// Get the [HttpStatus] of your Response
    pub const fn get_status(&self) -> &HttpStatus {
        &self.status
    }
    /// Get the body of your Response
    pub const fn get_body(&self) -> &String {
        &self.body
    }
    /// Set the body to a specific String
    pub fn set_body(&mut self, body: String) -> &mut Response {
        self.body = body;
        self
    }
    /// Set the version to as specific [HttpVersion]
    pub fn set_version(&mut self, version: HttpVersion) -> &mut Response {
        self.version = version;
        self
    }
    /// Set the status to as specific [HttpStatus]
    pub fn set_status(&mut self, status: HttpStatus) -> &mut Response {
        self.status = status;
        self
    }
    /// Add a single header to your Response
    pub fn add_header(&mut self, kv: (String, String)) -> &mut Response {
        self.headers.insert(kv.0, kv.1);
        self
    }
    /// Remove a specific Header from the Response (idempotent)
    pub fn remove_header(&mut self, key: &String) -> &mut Response {
        self.headers.remove(key);
        self
    }
    /// Get the header value to a specific key
    pub fn get_header(&mut self, key: &String) -> Option<&String> {
        self.headers.get(key)
    }
    /// Get the Headers as a mutable reference to manipulate it yourself
    pub fn get_headers_mut(&mut self) -> &mut BTreeMap<String, String> {
        &mut self.headers
    }
    /// Append the body by a string literatur
    pub fn append_body_str(&mut self, str: &str) -> &mut Response {
        self.body.push_str(str);
        self
    }
    /// Append the body by a String
    pub fn append_body(&mut self, str: String) -> &mut Response {
        self.append_body_str(str.as_str());
        self
    }
    fn parse_meta_line(str: Option<&str>) -> Result<(HttpVersion, HttpStatus), HttpParseError> {
        let mut split = str.ok_or(error_option_empty(Req))?
            .split(EMPTY_CHAR);
        let version = HttpVersion::try_from(split.next())?;
        let status = HttpStatus::try_from((
            split.next().ok_or(error_option_empty(Req))?,
            split.next().ok_or(error_option_empty(Req))?,
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

/// Builder impl for [Response]
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
pub struct ResponseBuilder {
    version: Option<HttpVersion>,
    status: Option<HttpStatus>,
    headers: Option<BTreeMap<String, String>>,
    body: Option<String>,
}

impl ResponseBuilder {
    /// validates if all the items are present
    pub const fn validate(&self) -> bool {
        self.body.is_some()
            && self.status.is_some()
            && self.headers.is_some()
            && self.version.is_some()
    }
    /// creates a new instance of ResponseBuilder with [None] values
    pub const fn new() -> Self {
        Self {
            body: None,
            status: None,
            headers: None,
            version: None,
        }
    }
    /// trys to make it to a [Response] otherwise returns a [HttpParseError]
    pub fn build(self) -> Result<Response, HttpParseError> {
        if !self.validate() {
            return Err(HttpParseError::from((Req, VALIDATE)));
        }
        Ok(Response {
            version: self.version.unwrap(),
            headers: self.headers.unwrap(),
            status: self.status.unwrap(),
            body: self.body.unwrap(),
        })
    }
    /// replaces the current value with the header parameter
    pub fn with_headers(mut self, headers: BTreeMap<String, String>) -> Self {
        self.headers = Some(headers);
        self
    }
    /// replaces the current value with the body parameter
    pub fn with_body(mut self, body: String) -> Self {
        self.body = Some(body);
        self
    }
    /// replaces the current value with the version parameter
    pub fn with_version(mut self, version: HttpVersion) -> Self {
        self.version = Some(version);
        self
    }
    /// replaces the current value with the status parameter
    pub fn with_status(mut self, status: HttpStatus) -> Self {
        self.status = Some(status);
        self
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl Destruct for ResponseBuilder {
    type Item = (Option<HttpVersion>, Option<HttpStatus>, Option<BTreeMap<String, String>>, Option<String>);
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
