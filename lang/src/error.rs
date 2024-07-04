use source_span::{
    fmt::{Formatter, Style},
    Position, Span,
};

use crate::{
    parser::type_def::TypeID,
    spanned::Spanned,
    tokenizer::{token::Token, Tokenizer},
};

pub type ParseResult<T> = Result<Spanned<T>, Error>;

#[derive(Debug)]
pub struct Error {
    kind: ErrorKind,
    span: Span,
}

#[derive(Debug)]
pub enum ErrorKind {
    // Parse Errors
    UnexpectedToken {
        found: Token,
        expected: Option<Token>,
    },
    InvalidOperator,
    UnexpectedEOF,

    // Execution Errors
    InvalidNumberOfArguments {
        expected: usize,
        found: usize,
    },
    VariableNotFound(String),
    FunctionNotFound(String),
    VariableAlreadyDeclared(String),

    TypeMismatch {
        expected: TypeID,
        found: TypeID,
    },

    NoMainFunction,
}

impl std::error::Error for ErrorKind {}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Error {
        Error { span, kind }
    }

    pub fn new_unexpected_token(
        Spanned::<Token> { value, span }: Spanned<Token>,
        expected: Option<Token>,
    ) -> Error {
        Error::new(
            span,
            ErrorKind::UnexpectedToken {
                found: value,
                expected,
            },
        )
    }

    pub fn new_invalid_number_of_arguments(span: Span, expected: usize, found: usize) -> Error {
        Error::new(
            span,
            ErrorKind::InvalidNumberOfArguments { expected, found },
        )
    }

    pub fn new_type_mismatch(span: Span, expected: TypeID, found: TypeID) -> Error {
        Error::new(span, ErrorKind::TypeMismatch { expected, found })
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn show_error<I>(&self, source: I)
    where
        I: Iterator<Item = Result<char, ()>>,
    {
        // Code to extract the source code from the span
        /*let source_buffer = SourceBuffer::new(
            source.chars().map(|c| Ok::<char, ()>(c)),
            Position::default(),
            Tokenizer::METRICS,
        );*/
        let message = format!("{}", self.kind);

        let full_span = Span::new(
            Position::default(),
            Position::new(usize::MAX - 1, usize::MAX - 1),
            Position::end(),
        );

        let mut fmt = Formatter::new();
        fmt.add(self.span, Some(message), Style::Error);

        let formatted = fmt.render(source, full_span, &Tokenizer::METRICS).unwrap();
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
            Self::InvalidNumberOfArguments {
                expected, found, ..
            } => write!(
                f,
                "Invalid number of arguments, expected {}, found {}",
                expected, found
            ),
            Self::VariableNotFound(name) => write!(f, "Variable '{}' was not found", name),
            Self::FunctionNotFound(name) => write!(f, "Function '{}' was not found", name),
            Self::VariableAlreadyDeclared(name) => {
                write!(f, "Variable '{}' already declared", name)
            }
            Self::TypeMismatch { expected, found } => {
                write!(
                    f,
                    "Type mismatch! Expected '{}', found '{}'",
                    expected, found
                )
            }
            Self::NoMainFunction => write!(f, "No main function found"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        std::error::Error::source(&self.kind)
    }
}
