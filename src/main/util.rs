use std::collections::BTreeMap;
use std::net::TcpStream;
use std::str::Lines;

use crate::{ParseErrorKind, Request};
use crate::error::HttpParseError;
use crate::error::ParseErrorKind::Util;

pub(crate) const KEY_VALUE_DELIMITER: &str = ": ";
pub(crate) const NEW_LINE: char = '\n';
pub(crate) const EMPTY_CHAR: char = ' ';
pub(crate) const OPTION_WAS_EMPTY: &str = "the Option<?> was empty and couldn't get unwrapped";
pub(crate) const INDEX_WAS_WRONG: &str = "The provided index didn't match";

pub(crate) trait ParseKeyValue {
    fn parse_key_value(&self) -> String;
}

impl ParseKeyValue for BTreeMap<String, String> {
    fn parse_key_value(&self) -> String {
        let mut string = String::new();
        for (key, value) in self {
            string.push_str(key);
            string.push_str(KEY_VALUE_DELIMITER);
            string.push_str(value);
            string.push(NEW_LINE);
        }
        string
    }
}

/// Trait for destructing structs with private fields.
/// It can also be used to run destroy logic <br>
///
/// Example:
/// ```notrust
///impl Destruct for ExampleType {
///    type Item = (u16, String);
///    fn destruct(self) -> Self::Item {
///        (self.code, self.message)
///    }
///}
/// ```
pub trait Destruct {
    /// used for specifying what kind of for example tuple
    /// will be returned from this function
    type Item;
    /// destructs the Struct into it's fields
    fn destruct(self) -> Self::Item;
}

pub(crate) fn parse_body(lines: &mut Lines) -> String {
    let mut string = String::new();
    lines.for_each(|str| {
        string.push_str(str);
        string.push(NEW_LINE);
    });
    string.remove((string.len() as isize - 1) as usize);
    string
}

pub(crate) fn parse_header(lines: &mut Lines) -> Result<BTreeMap<String, String>, HttpParseError> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut opt_line = lines.next();
    while opt_line.is_some() {
        let line = opt_line.unwrap();
        if !line.is_empty() {
            let (key, val) = parse_key_value(line)?;
            map.insert(key, val);
            opt_line = lines.next();
        } else {
            opt_line = None
        }
    }
    Ok(map)
}

pub(crate) fn parse_uri(str: Option<&str>) -> Result<String, HttpParseError> {
    str.ok_or(error_option_empty(Util)).map(String::from)
}

fn parse_key_value(str: &str) -> Result<(String, String), HttpParseError> {
    let mut key_value = str.split(KEY_VALUE_DELIMITER);
    let key = key_value
        .next()
        .ok_or(error_option_empty(Util))
        .map(String::from)?;
    let value = key_value
        .next()
        .ok_or(error_option_empty(Util))
        .map(String::from)?;
    Ok((key, value))
}

pub(crate) fn error_option_empty(kind: ParseErrorKind) -> HttpParseError {
    HttpParseError::from((kind, OPTION_WAS_EMPTY))
}

/// Trait for adding a method ro specific types to parse them automatically to a [Request]
pub trait TryRequest {
    /// trys to parse it to a [Request] otherwise returns a [HttpParseError]
    fn try_to_request(&mut self) -> Result<Request, HttpParseError>;
}

impl TryRequest for TcpStream {
    fn try_to_request(&mut self) -> Result<Request, HttpParseError> {
        Request::try_from(self)
    }
}