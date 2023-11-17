use std::fmt::{Debug, Display, Formatter};

const MESSAGE: &str = "Failure:";

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
        write!(f, "{:?}: {}", self, MESSAGES[*self as usize])
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct HttpParseError(ParseErrorKind);

impl HttpParseError {
    pub const fn new() -> Self {
        Self(ParseErrorKind::Unkown)
    }
    pub const fn get_kind(&self) -> &ParseErrorKind {
        &self.0
    }
}

impl From<ParseErrorKind> for HttpParseError {
    fn from(value: ParseErrorKind) -> Self {
        Self(value)
    }
}

impl Default for HttpParseError {
    fn default() -> Self {
        Self::new()
    }
}

impl Debug for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"{} {} ",self.0,MESSAGE)
    }
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
