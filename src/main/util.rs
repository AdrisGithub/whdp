use std::collections::BTreeMap;

pub const KEY_VALUE_DELIMITER: &str = ": ";
pub const NEW_LINE: char = '\n';

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
