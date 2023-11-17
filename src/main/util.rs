use std::collections::BTreeMap;
use std::str::Lines;

use crate::error::HttpParseError;
use crate::util;

pub const KEY_VALUE_DELIMITER: &str = ": ";
pub const NEW_LINE: char = '\n';
pub const EMPTY_CHAR: char = ' ';

pub trait ParseKeyValue {
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

pub trait Destruct {
    type Item;

    fn destruct(self) -> Self::Item;
}

pub fn parse_body(lines: &mut Lines) -> String {
    let mut string = String::new();
    lines.for_each(|str| {
        string.push_str(str);
        string.push(NEW_LINE);
    });
    string
}

pub fn parse_header(lines: &mut Lines) -> Result<BTreeMap<String, String>, HttpParseError> {
    let mut map: BTreeMap<String, String> = BTreeMap::new();
    let mut opt_line = lines.next();
    while opt_line.is_some() {
        let line = opt_line.ok_or(HttpParseError::new())?;
        if !line.is_empty() {
            let (key, val) = util::parse_key_value(line)?;
            map.insert(key, val);
            opt_line = lines.next();
        } else {
            opt_line = None
        }
    }
    Ok(map)
}

fn parse_key_value(str: &str) -> Result<(String, String), HttpParseError> {
    let mut key_value = str.split(KEY_VALUE_DELIMITER);
    let key = key_value
        .next()
        .ok_or(HttpParseError::new())
        .map(String::from)?;
    let value = key_value
        .next()
        .ok_or(HttpParseError::new())
        .map(String::from)?;
    Ok((key, value))
}
