use std::collections::BTreeMap;
use std::fmt::{Debug, Display, Formatter};
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::str::FromStr;

use wjp::{Deserialize, map, ParseError, Serialize, SerializeHelper, Values};

use crate::error::{HttpParseError, ParseErrorKind::Req};
use crate::method::HttpMethod;
use crate::util::{Destruct, EMPTY_CHAR, OPTION_WAS_EMPTY, parse_body, parse_header, parse_uri, ParseKeyValue};
use crate::version::HttpVersion;

/// Struct for representing a HTTP Request
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
            .map_err(|err| HttpParseError::from((Req, err.to_string())))?;
        Self::try_from(string)
    }
}

impl TryFrom<&mut TcpStream> for Request {
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

impl Request {
    fn parse_meta_data_line(
        str: Option<&str>,
    ) -> Result<(HttpMethod, String, HttpVersion), HttpParseError> {
        let mut split = str
            .ok_or(HttpParseError::from((Req, OPTION_WAS_EMPTY)))?.split(EMPTY_CHAR);
        Ok((
            HttpMethod::try_from(split.next())?,
            parse_uri(split.next())?,
            HttpVersion::try_from(split.next())?,
        ))
    }
    /// Get the [HttpMethod] of this Request
    pub fn get_method(&self) -> &HttpMethod {
        &self.method
    }
    /// Get the uri of this Request
    pub fn get_uri(&self) -> &String {
        &self.uri
    }
    /// Get the headers of this Request
    pub fn get_headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }
    /// Get the body of this Request
    pub fn get_body(&self) -> &String {
        &self.body
    }
    /// Get the body of this Request parsed to the Type T
    pub fn get_parsed_body<T: Deserialize>(&self) -> Result<T, ParseError> {
        T::deserialize_str(self.get_body().as_str())
    }
    /// Get the version of this Request
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

impl Destruct for Request {
    type Item = (HttpMethod, String, HttpVersion, BTreeMap<String, String>, String);
    fn destruct(self) -> Self::Item {
        (self.method, self.uri, self.version, self.headers, self.body)
    }
}

impl TryFrom<Values> for Request {
    type Error = ParseError;
    fn try_from(value: Values) -> Result<Self, Self::Error> {
        let mut struc = value.get_struct().ok_or(ParseError::new())?;
        let body = struc.map_val("body", String::try_from)?;
        let headers = struc.map_val("headers", BTreeMap::try_from)?;
        let method = struc.map_val("method", HttpMethod::try_from)?;
        let version = struc.map_val("version", HttpVersion::try_from)?;
        let uri = struc.map_val("uri", String::try_from)?;
        Ok(Self { body, headers, method, version, uri })
    }
}

impl Serialize for Request {
    fn serialize(&self) -> Values {
        Values::Struct(map!(
            ("version",&self.version),
            ("headers",&self.headers),
            ("body",&self.body),
            ("uri",&self.uri),
            ("method",&self.method)
        ))
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use wjp::Serialize;

    use crate::Request;

    #[test]
    pub fn test() {
        let string = read_to_string("src/resources/request.txt").unwrap();
        let req = Request::try_from(string).unwrap();
        println!("{:?}", req);
        println!();
        println!("{}", req.json());
    }
}