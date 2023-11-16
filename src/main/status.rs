use std::fmt::{Debug, Display, Formatter};

use crate::error::HttpParseError;

#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct HttpStatus {
    code: u16,
    message: String,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum HttpStatusGroup {
    Informational,
    Successful,
    Redirection,
    ClientError,
    ServerError,
    Unknown,
}

impl HttpStatus {
    pub fn get_group(&self) -> HttpStatusGroup {
        HttpStatusGroup::from(self.code as usize)
    }
    pub fn has_group(&self, group: &HttpStatusGroup) -> bool {
        self.get_group().eq(group)
    }
    pub const fn get_code(&self) -> &u16 {
        &self.code
    }
    pub const fn get_message(&self) -> &String {
        &self.message
    }
    pub fn destruct(self) -> (u16, String) {
        (self.code, self.message)
    }
}

impl From<(u16, String)> for HttpStatus {
    fn from(value: (u16, String)) -> Self {
        Self {
            code: value.0,
            message: value.1,
        }
    }
}
impl From<(u16, &str)> for HttpStatus {
    fn from(value: (u16, &str)) -> Self {
        Self::from((value.0, String::from(value.1)))
    }
}
impl TryFrom<(usize, &str)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (usize, &str)) -> Result<Self, Self::Error> {
        let size = u16::try_from(value.0).map_err(|_err| HttpParseError::new())?;
        Ok(Self::from((size, value.1)))
    }
}
impl TryFrom<(isize, &str)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (isize, &str)) -> Result<Self, Self::Error> {
        let size = usize::try_from(value.0).map_err(|_err| HttpParseError::new())?;
        Self::try_from((size, value.1))
    }
}
impl TryFrom<(isize, String)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (isize, String)) -> Result<Self, Self::Error> {
        let size = usize::try_from(value.0).map_err(|_err| HttpParseError::new())?;
        Self::try_from((size, value.1))
    }
}
impl TryFrom<(usize, String)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (usize, String)) -> Result<Self, Self::Error> {
        let size = u16::try_from(value.0).map_err(|_err| HttpParseError::new())?;
        Ok(Self::from((size, value.1)))
    }
}

impl From<&HttpStatus> for HttpStatusGroup {
    fn from(value: &HttpStatus) -> Self {
        Self::from(value.code as usize)
    }
}

impl TryFrom<isize> for HttpStatusGroup {
    type Error = HttpParseError;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let value = usize::try_from(value).map_err(|_err| HttpParseError::new())?;
        Ok(Self::from(value))
    }
}

impl From<usize> for HttpStatusGroup {
    fn from(value: usize) -> Self {
        match value {
            100..=199 => HttpStatusGroup::Informational,
            200..=299 => HttpStatusGroup::Successful,
            300..=399 => HttpStatusGroup::Redirection,
            400..=499 => HttpStatusGroup::ClientError,
            500..=599 => HttpStatusGroup::ServerError,
            _ => HttpStatusGroup::Unknown,
        }
    }
}

impl Display for HttpStatusGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.code, self.message)
    }
}

impl Debug for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}
