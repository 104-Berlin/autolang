pub type Result<T> = std::result::Result<T, Error>;

pub struct Error {
    kind: ErrorKind,
    message: String,
}

pub enum ErrorKind {
    UnexpectedToken,
    UnexpectedEndOfInput,
    InvalidToken,

    InternalParserError(nom::error::ErrorKind),
}

impl Error {
    pub fn new(kind: ErrorKind, message: String) -> Error {
        Error { kind, message }
    }

    pub fn new_unexpected_token(message: String) -> Error {
        Error::new(ErrorKind::UnexpectedToken, message)
    }

    pub fn new_unexpected_end_of_input(message: String) -> Error {
        Error::new(ErrorKind::UnexpectedEndOfInput, message)
    }

    pub fn new_invalid_token(message: String) -> Error {
        Error::new(ErrorKind::InvalidToken, message)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ErrorKind::UnexpectedToken => write!(f, "Unexpected token"),
            ErrorKind::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ErrorKind::InvalidToken => write!(f, "Invalid token"),
            ErrorKind::InternalParserError(kind) => write!(f, "Internal parser error: {:?}", kind),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}: {}", self.kind, self.message)
    }
}

impl std::error::Error for Error {}

impl From<nom::Err<nom::error::Error<&str>>> for Error {
    fn from(err: nom::Err<nom::error::Error<&str>>) -> Error {
        match err {
            nom::Err::Failure(e) => {
                Error::new(ErrorKind::InternalParserError(e.code), "Failure".into())
            }
            nom::Err::Error(e) => {
                Error::new(ErrorKind::InternalParserError(e.code), "Error".into())
            }
            nom::Err::Incomplete(_) => {
                Error::new(ErrorKind::UnexpectedEndOfInput, "Incomplete".into())
            }
        }
    }
}
