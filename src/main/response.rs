use std::collections::BTreeMap;
use std::fmt::{Display, Formatter};

use crate::status::HttpStatus;
use crate::util::ParseKeyValue;
use crate::version::HttpVersion;

pub struct Response {
    version: HttpVersion,
    status: HttpStatus,
    headers: BTreeMap<String, String>,
    body: String,
}

impl Response {
    pub const fn get_version(&self) -> &HttpVersion {
        &self.version
    }
    pub const fn get_headers(&self) -> &BTreeMap<String, String> {
        &self.headers
    }
    pub const fn get_status(&self) -> &HttpStatus {
        &self.status
    }
    pub const fn get_body(&self) -> &String {
        &self.body
    }
    pub fn destruct(self) -> (HttpVersion, HttpStatus, BTreeMap<String, String>, String) {
        (self.version, self.status, self.headers, self.body)
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
