use source_span::{
    fmt::{Color, Formatter, Style},
    Span,
};

use crate::tokenizer::{TokenKind, Tokenizer};

pub type ParseResult<T> = Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

#[derive(Debug)]
pub enum ErrorKind {
    UnexpectedToken {
        found: TokenKind,
        expected: Option<TokenKind>,
    },
    InvalidOperator,

    UnexpectedEOF,
}

impl std::error::Error for ErrorKind {}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Error {
        Error { span, kind }
    }

    pub fn unexpected_token(span: Span, found: TokenKind, expected: Option<TokenKind>) -> Error {
        Error::new(span, ErrorKind::UnexpectedToken { found, expected })
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn show_error(&self, source: &str) {
        // Code to extract the source code from the span
        /*let source_buffer = SourceBuffer::new(
            source.chars().map(|c| Ok::<char, ()>(c)),
            Position::default(),
            Tokenizer::METRICS,
        );*/
        let message = format!("{}", self.kind);

        let mut fmt = Formatter::with_margin_color(Color::Blue);
        fmt.add(self.span, Some(message), Style::Error);
        let formatted = fmt
            .render(
                source.chars().map(|c| Ok::<char, ()>(c)),
                self.span.aligned(),
                &Tokenizer::METRICS,
            )
            .unwrap();

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
            Self::UnexpectedToken {
                found,
                expected: None,
            } => write!(f, "Unexpected token '{}'", found),
            Self::UnexpectedToken {
                found,
                expected: Some(expected),
            } => {
                write!(f, "Unexpected token '{}', expected '{}'", found, expected)
            }
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
