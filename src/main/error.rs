use std::fmt::{Debug, Display, Formatter};

const MESSAGES: [&str; 6] = [
    "Unknown Failure at parsing the Request",
    "Failure at a IO operation",
    "Failure at parsing the HTTP Method",
    "Failure at parsing the HTTP Version",
    "Failure at parsing the HTTP Request",
    "Failure at parsing the HTTP Status",
];

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub enum ParseErrorKind {
    Unkown,
    IO,
    Method,
    Version,
    Request,
    Status,
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
        f.write_str(&self.0.to_string())
    }
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
