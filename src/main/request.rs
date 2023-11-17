use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::FromStr;

use crate::error::{HttpParseError, ParseErrorKind::Request as ReqError};
use crate::method::HttpMethod;
use crate::util::{Destruct, EMPTY_CHAR, parse_body, parse_header,parse_uri,ParseKeyValue};
use crate::version::HttpVersion;

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Request {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: BTreeMap<String, String>,
    body: String,
}

impl<'a> TryFrom<&'a str> for Request {
    type Error = HttpParseError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl FromStr for Request {
    type Err = HttpParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (method, uri, version) = Self::parse_meta_data_line(lines.next())?;
        let headers = parse_header(&mut lines)?;
        let body = parse_body(&mut lines);
        Ok(Self {
            method,
            uri,
            version,
            headers,
            body,
        })
    }
}

impl TryFrom<String> for Request {
    type Error = HttpParseError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_str(value.as_str())
    }
}

impl TryFrom<&[u8]> for Request {
    type Error = HttpParseError;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Self::try_from(Vec::from(value))
    }
}

impl TryFrom<Vec<u8>> for Request {
    type Error = HttpParseError;
    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let string = String::from_utf8(value)
            .map_err(|_a| HttpParseError::from(ReqError))?;
        Self::try_from(string)
    }
}

impl TryFrom<&mut TcpStream> for Request {
    type Error = HttpParseError;
    fn try_from(value: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let received: Vec<u8> = reader
            .fill_buf()
            .map_err(|_err| HttpParseError::from(ReqError))?
            .to_vec();
        reader.consume(received.len());
        Self::try_from(received)
    }
}

impl Request {
    fn parse_meta_data_line(
        str: Option<&str>,
    ) -> Result<(HttpMethod, String, HttpVersion), HttpParseError> {
        let mut split = str.ok_or(HttpParseError::from(ReqError))?.split(EMPTY_CHAR);
        Ok((
            HttpMethod::try_from(split.next())?,
            parse_uri(split.next())?,
            HttpVersion::try_from(split.next())?,
        ))
    }
    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }
    pub fn get_uri(&self) -> &String {
        &self.uri
    }
    pub fn get_headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }
    pub fn get_body(&self) -> &String {
        &self.body
    }
    pub fn get_version(&self) -> &HttpVersion {
        &self.version
    }
}

impl Debug for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} \n{}\n{}",
            self.method,
            self.uri,
            self.version,
            self.headers.parse_key_value(),
            self.body
        )
    }
}

impl Display for Request {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

pub trait TryRequest {
    fn try_to_request(&mut self) -> Result<Request, HttpParseError>;
}

impl TryRequest for TcpStream {
    fn try_to_request(&mut self) -> Result<Request, HttpParseError> {
        Request::try_from(self)
    }
}

impl Destruct for Request {
    type Item = (HttpMethod, String, HttpVersion, BTreeMap<String, String>, String);
    fn destruct(self) -> Self::Item {
        (self.method, self.uri, self.version, self.headers, self.body)
    }
}
