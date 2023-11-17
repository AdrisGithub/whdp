use std::fmt::{Debug, Display, Formatter};

use crate::error::ParseErrorKind::Unkown;
use crate::util::Destruct;

const MESSAGE: &str = "Failure:";
const UNKNOWN_MSG: &str = "Unkown";

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum ParseErrorKind {
    Unkown,
    IO,
    Method,
    Version,
    Request,
    Status,
    Util,
    Response,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct HttpParseError {
    kind: ParseErrorKind,
    msg: Option<String>,
}

impl HttpParseError {
    pub const fn new() -> Self {
        Self {
            kind: Unkown,
            msg: None,
        }
    }
    pub const fn get_kind(&self) -> &ParseErrorKind {
        &self.kind
    }
    pub fn with_msg(&mut self, str: String) -> &mut Self {
        self.msg = Some(str);
        self
    }
    pub fn get_msg(&self) -> String {
        self.msg.clone().unwrap_or(UNKNOWN_MSG.into())
    }
}

impl From<ParseErrorKind> for HttpParseError {
    fn from(value: ParseErrorKind) -> Self {
        Self {
            kind: value,
            msg: None,
        }
    }
}

impl From<(ParseErrorKind, &str)> for HttpParseError {
    fn from(value: (ParseErrorKind, &str)) -> Self {
        Self::from((value.0, String::from(value.1)))
    }
}

impl From<(ParseErrorKind, String)> for HttpParseError {
    fn from(value: (ParseErrorKind, String)) -> Self {
        let mut err = Self::from(value.0);
        err.with_msg(value.1);
        err
    }
}

impl Default for HttpParseError {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}{}", self.kind, MESSAGE, self.get_msg())
    }
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Destruct for HttpParseError {
    type Item = (ParseErrorKind, String);
    fn destruct(self) -> Self::Item {
        (self.kind, self.get_msg())
    }
}
