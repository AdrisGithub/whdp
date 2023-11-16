use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::{FromStr, Lines};

use crate::error::HttpParseError;
use crate::method::HttpMethod;
use crate::version::HttpVersion;

const KEY_VALUE_DELIMITER: &str = ": ";
const NEW_LINE: char = '\n';

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
        let headers = Self::parse_header(&mut lines)?;
        let body = Self::parse_body(&mut lines);
        Ok(
            Self {
                method,
                uri,
                version,
                headers,
                body,
            }
        )
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
            .map_err(|_a| HttpParseError::new())?;
        Self::try_from(string)
    }
}

impl TryFrom<&mut TcpStream> for Request {
    type Error = HttpParseError;


    fn try_from(value: &mut TcpStream) -> Result<Self, Self::Error> {
        let mut reader = BufReader::new(value);
        let received: Vec<u8> = reader.fill_buf()?
            .to_vec();
        reader.consume(received.len());
        Self::try_from(received)
    }
}

impl Request {
    fn parse_method(str: Option<&str>) -> Result<HttpMethod, HttpParseError> {
        str.ok_or(HttpParseError::new())
            .map(HttpMethod::from_str)?
    }
    fn parse_uri(str: Option<&str>) -> Result<String, HttpParseError> {
        str.ok_or(HttpParseError::new())
            .map(String::from)
    }
    fn parse_version(str: Option<&str>) -> Result<HttpVersion, HttpParseError> {
        str.ok_or(HttpParseError::new())
            .map(HttpVersion::from_str)?
    }
    fn parse_body(lines: &mut Lines) -> String {
        let mut string = String::new();
        lines.for_each(|str| string.push_str(str));
        string
    }
    fn parse_meta_data_line(str: Option<&str>) -> Result<(HttpMethod, String, HttpVersion), HttpParseError> {
        let mut split = str.ok_or(HttpParseError::new())?.split(' ');
        Ok((
            Self::parse_method(split.next())?,
            Self::parse_uri(split.next())?,
            Self::parse_version(split.next())?
        ))
    }
    fn parse_header(lines: &mut Lines) -> Result<BTreeMap<String, String>, HttpParseError> {
        let mut map: BTreeMap<String, String> = BTreeMap::new();
        let mut opt_line = lines.next();
        while opt_line.is_some() {
            let line = opt_line.ok_or(HttpParseError::new())?;
            if !line.is_empty() {
                let (key, val) = Self::parse_key_value(line)?;
                map.insert(key, val);
                opt_line = lines.next();
            } else { opt_line = None }
        }
        Ok(map)
    }
    fn parse_key_value(str: &str) -> Result<(String, String), HttpParseError> {
        let mut key_value = str.split(KEY_VALUE_DELIMITER);
        let key = key_value.next()
            .ok_or(HttpParseError::new())
            .map(String::from)?;
        let value = key_value.next()
            .ok_or(HttpParseError::new())
            .map(String::from)?;
        Ok((key, value))
    }
    fn headers_to_string(&self) -> String {
        let mut string = String::new();
        for (key, value) in &self.headers {
            string.push_str(key);
            string.push_str(KEY_VALUE_DELIMITER);
            string.push_str(value);
            string.push(NEW_LINE);
        }
        string
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
        write!(f, "{} {} {} \n{}\n{}", self.method, self.uri, self.version, self.headers_to_string(), self.body)
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