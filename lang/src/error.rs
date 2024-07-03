use source_span::Span;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken,
    InvalidOperator,

    UnexpectedEOF,
}

impl std::error::Error for ErrorKind {}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Error {
        Error { span, kind }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.span.start().line,
            self.span.start().column,
            self.kind
        )
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UnexpectedToken => write!(f, "Unexpected token"),
            Self::InvalidOperator => write!(f, "Invalid operator"),
            Self::UnexpectedEOF => write!(f, "Unexpected end of file"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        std::error::Error::source(&self.kind)
    }
}
