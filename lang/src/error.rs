use source_span::{fmt::{Color, Formatter, Style}, Span};

use crate::tokenizer::Tokenizer;

pub type ParseResult<T> = Result<T, Error>;

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

    pub fn show_error(&self, source: &str) {
        let mut fmt = Formatter::with_margin_color(Color::Blue);
        fmt.add(self.span, None, Style::Error);
        let formatted = fmt.render(source.chars().map(|c| Ok::<char, Error>(c)), self.span.aligned(), &Tokenizer::METRICS).unwrap();
        println!("Error: {}", self.kind);
        println!("{}", formatted);
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
