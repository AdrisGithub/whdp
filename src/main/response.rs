use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::FromStr;

use wjp::{Deserialize, map, ParseError, Serialize, SerializeHelper, Values};

use crate::error::{HttpParseError, ParseErrorKind::Req};
use crate::status::HttpStatus;
use crate::status::status_presets::ok;
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
    /// Get the body parsed to the Parameter T
    pub fn get_parsed_body<T: Deserialize>(&self) -> Result<T, ParseError> {
        T::deserialize_str(self.body.as_str())
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
impl TryFrom<&mut TcpStream> for Response{
    type Error = HttpParseError;
    fn try_from(value: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let received: Vec<u8> = reader
            .fill_buf()
            .map_err(|err| HttpParseError::from((Req, err.to_string())))?
            .to_vec();
        reader.consume(received.len());
        Self::try_from(received)
    }
}
impl TryFrom<Vec<u8>> for Response {
    type Error = HttpParseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let string = String::from_utf8(value)
            .map_err(|err| HttpParseError::from((Req, err.to_string())))?;
        Self::try_from(string)
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
    /// replaces the current body with a [`serializable`] Body
    ///
    /// [`serializable`]: Serialize
    pub fn with_body_ser<T: Serialize>(self, body: T) -> Self {
        self.with_body(body.json())
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
    /// replaces the current value with empty header
    pub fn with_empty_headers(self) -> Self {
        self.with_headers(BTreeMap::new())
    }
    // replaces the current value with an empty body
    pub fn with_empty_body(self) -> Self {
        self.with_body(String::new())
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<Values> for Response {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let body = struc.map_val("body", String::try_from)?;
        let headers = struc.map_val("headers", BTreeMap::try_from)?;
        let status = struc.map_val("status", HttpStatus::try_from)?;
        let version = struc.map_val("version", HttpVersion::try_from)?;
        Ok(Self { body, headers, status, version })
    }
}

impl Serialize for Response {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("body",self.body.serialize()),
            ("headers",self.headers.serialize()),
            ("status",self.status.serialize()),
            ("version",self.version.serialize())
        ))
    }
}

impl Destruct for ResponseBuilder {
    type Item = (Option<HttpVersion>, Option<HttpStatus>, Option<BTreeMap<String, String>>, Option<String>);
    fn destruct(self) -> Self::Item {
        (self.version, self.status, self.headers, self.body)
    }
}

/// Several presets for standard Responses
pub mod resp_presets {
    use crate::{HttpStatus, Response, ResponseBuilder, status_presets};
    use crate::HttpVersion::OnePointOne;

    /// creates an empty [Response] with version 1.1 and the given [HttpStatus]
    pub fn from_status(status: HttpStatus) -> Response {
        ResponseBuilder::new()
            .with_empty_body()
            .with_empty_headers()
            .with_version(OnePointOne)
            .with_status(status)
            .build().unwrap()
    }

    /// creates a [Response] with version 1.1, empty headers, the given [HttpStatus] and a given body
    pub fn from_status_and_body(status: HttpStatus, body: String) -> Response {
        let mut resp = from_status(status);
        resp.set_body(body);
        resp
    }

    /// uses the [from_status_and_body] method to create a Response with Status Continue
    pub fn r#continue(str: String) -> Response {
        from_status_and_body(status_presets::r#continue(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status OK
    pub fn ok(str: String) -> Response {
        from_status_and_body(status_presets::ok(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status Bad Request
    pub fn bad_request(str: String) -> Response {
        from_status_and_body(status_presets::bad_request(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status Not Found
    pub fn not_found(str: String) -> Response {
        from_status_and_body(status_presets::not_found(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status Created
    pub fn created(str: String) -> Response {
        from_status_and_body(status_presets::created(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status Internal Server Error
    pub fn internal_server_error(str: String) -> Response {
        from_status_and_body(status_presets::internal_server_error(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status No Content
    pub fn no_content(str: String) -> Response {
        from_status_and_body(status_presets::no_content(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status Not implemented
    pub fn not_implemented(str: String) -> Response {
        from_status_and_body(status_presets::not_implemented(), str)
    }

    /// uses the [from_status_and_body] method to create a Response with Status Unsupported Media Type
    pub fn unsupported_media_type(str: String) -> Response {
        from_status_and_body(status_presets::unsupported_media_type(), str)
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;
    use wjp::Serialize;

    use crate::response::Response;

    #[test]
    fn test() {
        let string = read_to_string("src/resources/response.txt").unwrap();
        let resp = Response::try_from(string).unwrap();
        println!("{:?}", resp);
        println!();
        println!("{}", resp.json());
    }
}
