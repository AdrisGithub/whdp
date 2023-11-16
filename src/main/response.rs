use std::collections::BTreeMap;

use crate::status::HttpStatus;
use crate::version::HttpVersion;

pub struct Response {
    version: HttpVersion,
    status: HttpStatus,
    headers: BTreeMap<String, String>,
    body: String,
}
