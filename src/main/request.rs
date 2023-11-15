use std::collections::HashMap;
use std::str::FromStr;

use crate::error::HttpParseError;
use crate::method::HttpMethod;
use crate::version::HttpVersion;

#[derive(Clone, Eq, Debug, PartialEq)]
pub struct Request<'a> {
    method: HttpMethod,
    uri: &'a str,
    version: HttpVersion,
    headers: HashMap<&'a str, &'a str>,
    body: &'a str,
}

impl<'b, 'a: 'b> TryFrom<&'a str> for Request<'b> {
    type Error = HttpParseError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        Self::from_string(value)
    }
}

impl<'a> Request<'a> {
    fn from_string<'b>(str: &'b str) -> Result<Request<'a>, HttpParseError> where 'b: 'a {
        let mut lines = str.lines();
        let (method, uri, version) = Self::parse_meta_data_line(lines.next())?;
        let headers = Self::parse_header(lines.next())?;
        let body = lines.next().ok_or(HttpParseError::new())?;
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
    fn parse_method(str: Option<&str>) -> Result<HttpMethod, HttpParseError> {
        str.ok_or(HttpParseError::new())
            .map(HttpMethod::from_str)?
    }
    fn parse_uri(str: Option<&str>) -> Result<&str, HttpParseError> {
        str.ok_or(HttpParseError::new())
    }
    fn parse_version(str: Option<&str>) -> Result<HttpVersion, HttpParseError> {
        str.ok_or(HttpParseError::new())
            .map(HttpVersion::from_str)?
    }
    fn parse_meta_data_line(str: Option<&str>) -> Result<(HttpMethod, &str, HttpVersion), HttpParseError> {
        let mut split = str.ok_or(HttpParseError::new())?.split(' ');
        Ok((
            Self::parse_method(split.next())?,
            Self::parse_uri(split.next())?,
            Self::parse_version(split.next())?
        ))
    }
    fn parse_header(str: Option<&str>) -> Result<HashMap<&str, &str>, HttpParseError> {
        let mut map: HashMap<&str, &str> = HashMap::new();
        let str = str.ok_or(HttpParseError::new())?
            .lines()
            .map(Self::parse_key_value);
        for entry in str {
            let (key, value) = entry?;
            map.insert(key, value);
        }
        Ok(map)
    }
    fn parse_key_value(str: &str) -> Result<(&str, &str), HttpParseError> {
        let mut key_value = str.split(':');
        let key = key_value.next().ok_or(HttpParseError::new())?;
        let value = key_value.next().ok_or(HttpParseError::new())?;
        Ok((key, value))
    }
}