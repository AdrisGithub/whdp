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


fn parse_header_line(str: &str) -> Result<Test, HttpParseError> {
    let mut lines = str.split(' ');
    let method = lines.next()
        .ok_or(HttpParseError::new())
        .map(HttpMethod::from_str)?;
    let uri = lines.next()
        .ok_or(HttpParseError::new())
        .map(String::from);
    let version = lines.next()
        .ok_or(HttpParseError::new())
        .map(HttpVersion::from_str)?;
    Ok(Test {
        method: method?,
        uri: uri?,
        version: version?,
    })
}

struct Test {
    method: HttpMethod,
    uri: String,
    version: HttpVersion,
}