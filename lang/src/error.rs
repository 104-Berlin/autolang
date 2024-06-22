use std::num::ParseIntError;

use nom::error::{ContextError, FromExternalError, ParseError};

use crate::parser::spans::{InputSpan, LocatedSpan, Location};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    inner: Location<ErrorKind>,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken,
    InvalidOperator,

    UnexpectedEOF,

    Other(nom::error::ErrorKind),
}

impl std::error::Error for ErrorKind {}

impl Error {
    pub fn new(span: InputSpan, kind: ErrorKind) -> Error {
        Error {
            inner: LocatedSpan::from(span)
                .map_fragment(str::len)
                .copy_with_extra(kind),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.inner.extra
    }

    pub fn span(&self) -> Location {
        self.inner.with_no_extra()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.inner.location_line(),
            self.inner.get_column(),
            self.inner.extra
        )
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken => write!(f, "Unexpected token"),
            Self::InvalidOperator => write!(f, "Invalid operator"),
            Self::UnexpectedEOF => write!(f, "Unexpected end of file"),
            Self::Other(kind) => write!(f, "Internal parser error: {:?}", kind),
        }
    }
}

impl ParseError<InputSpan<'_>> for Error {
    fn from_error_kind(input: InputSpan, kind: nom::error::ErrorKind) -> Self {
        Error::new(input, ErrorKind::Other(kind))
    }

    fn append(_: InputSpan, _: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}

impl ContextError<InputSpan<'_>> for Error {
    fn add_context(_input: InputSpan<'_>, _ctx: &'static str, other: Self) -> Self {
        other
    }
}

impl FromExternalError<InputSpan<'_>, ErrorKind> for Error {
    fn from_external_error(input: InputSpan<'_>, _: nom::error::ErrorKind, e: ErrorKind) -> Self {
        Error::new(input, e)
    }
}

impl FromExternalError<InputSpan<'_>, ParseIntError> for Error {
    fn from_external_error(
        input: InputSpan<'_>,
        _: nom::error::ErrorKind,
        _: ParseIntError,
    ) -> Self {
        Error::new(input, ErrorKind::Other(nom::error::ErrorKind::Digit))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        std::error::Error::source(&self.inner.extra)
    }
}
