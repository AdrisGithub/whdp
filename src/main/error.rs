use std::fmt::{Debug, Display, Formatter};

use crate::util::Destruct;

const MESSAGE: &str = "Failure:";
const UNKNOWN_MSG: &str = "Something unexpected went wrong";

/// ### Error struct for HTTP Parsing
///
/// contains a [kind] for automatically handling the error <br>
/// and an optional [message] for further information
///
/// [kind]: crate::HttpParseError::get_kind
/// [message]: crate::HttpParseError::get_msg
#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Hash)]
pub struct HttpParseError {
    kind: ParseErrorKind,
    msg: Option<String>,
}

impl HttpParseError {
    /// constructs a new instance of HttpParseError
    /// with default values
    pub const fn new() -> Self {
        Self {
            kind: ParseErrorKind::Unkown,
            msg: None,
        }
    }
    /// get the [ParseErrorKind] of this Error
    pub const fn get_kind(&self) -> &ParseErrorKind {
        &self.kind
    }
    /// change the Error to have this message
    pub fn with_msg(&mut self, str: String) -> &mut Self {
        self.msg = Some(str);
        self
    }
    /// get the Message of this Error
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

/// #### Enum for the different places where the parsing could went wrong
/// This is more for error handling in match cases. It's used in [HttpParseError] <br>
/// For genuine Information where it went wrong read the message
#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum ParseErrorKind {
    /// Error type for [Default] Error
    Unkown,
    /// Error type for everything that has to do with parsing the [HttpMethod]
    ///
    /// [HttpMethod]: crate::HttpMethod
    Method,
    /// Error type for everything that has to do with parsing the [HttpVersion]
    ///
    /// [HttpVersion]: crate::HttpVersion
    Version,
    /// Error type for everything that has to do with parsing the [HttpRequest]
    ///
    /// [HttpRequest]: crate::Request
    Req,
    /// Error type for everything that has to do with parsing the [HttpStatus]
    ///
    /// [HttpStatus]: crate::HttpStatus
    Status,
    /// Error type for everything that has to do with parsing the [HttpResponse]
    ///
    /// [HttpResponse]: crate::Response
    Resp,
    /// Error type for some util functions for example
    /// 1. Parsing the body
    /// 2. Parsing the headers
    /// 3. Parsing the uri
    Util,
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
