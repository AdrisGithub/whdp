use std::collections::HashMap;
use std::str::FromStr;

use crate::error::HttpParseError;
use crate::method::HttpMethod;
use crate::version::HttpVersion;

pub struct Request {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
    headers: HashMap<String, String>,
    body: String,
}

impl FromStr for Request {
    type Err = HttpParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let (method, uri, version) = Self::parse_meta_data_line(lines.next())?;
        let headers = Self::parse_header(lines.next())?;
        let body = lines.next().ok_or(HttpParseError::new())?.to_string();
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
    fn parse_meta_data_line(str: Option<&str>) -> Result<(HttpMethod, String, HttpVersion), HttpParseError> {
        let mut split = str.ok_or(HttpParseError::new())?.split(' ');
        Ok((
            Self::parse_method(split.next())?,
            Self::parse_uri(split.next())?,
            Self::parse_version(split.next())?
        ))
    }
    fn parse_header(str: Option<&str>) -> Result<HashMap<String, String>, HttpParseError> {
        let mut map: HashMap<String, String> = HashMap::new();
        let str = str.ok_or(HttpParseError::new())?
            .lines()
            .map(Self::parse_key_value);
        for entry in str {
            let (key, value) = entry?;
            map.insert(String::from(key), String::from(value));
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