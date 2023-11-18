use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

use crate::error::HttpParseError;
use crate::error::ParseErrorKind::Status;
use crate::util::{Destruct, EMPTY_CHAR, OPTION_WAS_EMPTY};

/// Struct for HTTP Status Codes
#[derive(Ord, PartialOrd, Eq, PartialEq, Clone, Hash)]
pub struct HttpStatus {
    code: u16,
    message: String,
}

impl HttpStatus {
    /// gets the associated [HttpStatusGroup]
    pub fn get_group(&self) -> HttpStatusGroup {
        HttpStatusGroup::from(self.code as usize)
    }
    /// looks if the own [HttpStatusGroup] is the same as the given one
    pub fn has_group(&self, group: &HttpStatusGroup) -> bool {
        self.get_group().eq(group)
    }
    /// returns the HTTP Status Code
    pub const fn get_code(&self) -> &u16 {
        &self.code
    }
    /// returns the name associated with its Status Code
    pub const fn get_message(&self) -> &String {
        &self.message
    }
}

impl Destruct for HttpStatus {
    type Item = (u16, String);
    fn destruct(self) -> Self::Item {
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
        let size = u16::try_from(value.0)
            .map_err(|err| HttpParseError::from((Status, err.to_string())))?;
        Ok(Self::from((size, value.1)))
    }
}

impl TryFrom<(isize, &str)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (isize, &str)) -> Result<Self, Self::Error> {
        let size = usize::try_from(value.0)
            .map_err(|err| HttpParseError::from((Status, err.to_string())))?;
        Self::try_from((size, value.1))
    }
}

impl TryFrom<(isize, String)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (isize, String)) -> Result<Self, Self::Error> {
        let size = usize::try_from(value.0)
            .map_err(|err| HttpParseError::from((Status, err.to_string())))?;
        Self::try_from((size, value.1))
    }
}

impl TryFrom<(usize, String)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (usize, String)) -> Result<Self, Self::Error> {
        let size = u16::try_from(value.0)
            .map_err(|err| HttpParseError::from((Status, err.to_string())))?;
        Ok(Self::from((size, value.1)))
    }
}

impl TryFrom<(&str, &str)> for HttpStatus {
    type Error = HttpParseError;
    fn try_from(value: (&str, &str)) -> Result<Self, Self::Error> {
        let code = u16::from_str(value.0)
            .map_err(|err| HttpParseError::from((Status, err.to_string())))?;
        Ok(Self::from((code, value.1)))
    }
}

impl FromStr for HttpStatus {
    type Err = HttpParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(EMPTY_CHAR);
        let first = split.next()
            .ok_or(HttpParseError::from((Status, OPTION_WAS_EMPTY)))?;
        let second = split.next()
            .ok_or(HttpParseError::from((Status, OPTION_WAS_EMPTY)))?;
        Self::try_from((first, second))
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

/// Enum for HTTP Status Codes Groups
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum HttpStatusGroup {
    /// between 100 - 199
    Informational,
    /// between 200 - 299
    Successful,
    /// between 300 - 399
    Redirection,
    /// between 400 - 499
    ClientError,
    /// between 500 - 599
    ServerError,
    /// incase self-made HTTP Status is invalid
    Unknown,
}

impl From<&HttpStatus> for HttpStatusGroup {
    fn from(value: &HttpStatus) -> Self {
        Self::from(value.code as usize)
    }
}

impl TryFrom<isize> for HttpStatusGroup {
    type Error = HttpParseError;
    fn try_from(value: isize) -> Result<Self, Self::Error> {
        let value = usize::try_from(value)
            .map_err(|err| HttpParseError::from((Status, err.to_string())))?;
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


/// Several preset Status Codes like [OK], [Created], [Not Found]
///
/// [OK]: crate::presets::ok
/// [Created]: crate::presets::created
/// [Not Found]: crate::presets::not_found
pub mod presets {
    use crate::status::HttpStatus;

    /// preset for the Status code [100]
    ///
    /// [100]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/100
    pub fn r#continue() -> HttpStatus {
        HttpStatus::from((100, "Continue"))
    }

    /// preset for the Status code [200]
    ///
    /// [200]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/200
    pub fn ok() -> HttpStatus {
        HttpStatus::from((200, "OK"))
    }

    /// preset for the Status code [201]
    ///
    /// [201]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/201
    pub fn created() -> HttpStatus {
        HttpStatus::from((201, "Created"))
    }

    /// preset for the Status code [204]
    ///
    /// [204]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/204
    pub fn no_content() -> HttpStatus {
        HttpStatus::from((204, "No Content"))
    }

    /// preset for the Status code [400]
    ///
    /// [400]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400
    pub fn bad_request() -> HttpStatus {
        HttpStatus::from((400, "Bad Request"))
    }

    /// preset for the Status code [404]
    ///
    /// [404]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/400
    pub fn not_found() -> HttpStatus {
        HttpStatus::from((404, "Not Found"))
    }

    /// preset for the Status code [415]
    ///
    /// [415]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/415
    pub fn unsupported_media_type() -> HttpStatus {
        HttpStatus::from((415, "Unsupported Media Type"))
    }

    /// preset for the Status code [500]
    ///
    /// [500]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500
    pub fn internal_server_error() -> HttpStatus {
        HttpStatus::from((500, "Internal Server Error"))
    }

    /// preset for the Status code [501]
    ///
    /// [501]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/501
    pub fn not_implemented() -> HttpStatus {
        HttpStatus::from((501, "Not Implemented"))
    }
}
