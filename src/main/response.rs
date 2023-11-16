use crate::status::HttpStatus;
use crate::version::HttpVersion;
use std::collections::BTreeMap;

pub struct Response {
    version: HttpVersion,
    status: HttpStatus,
    headers: BTreeMap<String, String>,
    body: String,
}
