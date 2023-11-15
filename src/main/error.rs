use std::fmt::{Debug, Display, Formatter};

const MESSAGE: &str = "Failure at parsing";

// TODO better error messages
#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone, Hash)]
pub struct HttpParseError(());

impl HttpParseError {
    pub const fn new() -> Self {
        Self(())
    }
}

impl Default for HttpParseError {
    fn default() -> Self {
        Self::new()
    }
}


impl Debug for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(MESSAGE)
    }
}

impl Display for HttpParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}