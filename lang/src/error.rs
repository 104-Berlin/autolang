use source_span::{
    fmt::{Formatter, Style},
    Position, Span,
};

use crate::{
    execution::value::Value,
    parser::{
        binary_expression::BinaryOperator,
        type_def::{TypeDef, TypeID},
    },
    spanned::Spanned,
    tokenizer::{token::Token, Tokenizer},
};

pub type ALResult<T> = Result<Spanned<T>, Error>;

#[derive(Debug)]
pub struct Error {
    kind: Box<ErrorKind>,
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
    InvalidAssignmentTarget,

    TypeMismatch {
        expected: TypeID,
        found: TypeID,
        reason: TypeMismatchReason,
    },
    TypeNotFound(String),

    StructFieldNotInitialized(String),
    StructFieldNotFound(String),

    FailedToAccessField(TypeDef),

    NoMainFunction,

    // For controll flow
    Return(Value),
    Break,
    Continue,
}

#[derive(Debug, Clone)]
pub enum TypeMismatchReason {
    FunctionReturn,
    FunctionArgument,
    BinaryOperation(BinaryOperator),
    VariableAssignment,
}

impl std::error::Error for ErrorKind {}

impl Error {
    pub fn new(span: Span, kind: ErrorKind) -> Error {
        Error { span, kind: Box::new(kind) }
    }

    pub fn with_span(self) -> Self {
        Self {
            span: self.span,
            ..self
        }
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

    pub fn new_type_mismatch(
        span: Span,
        expected: TypeID,
        found: TypeID,
        reason: TypeMismatchReason,
    ) -> Error {
        Error::new(
            span,
            ErrorKind::TypeMismatch {
                expected,
                found,
                reason,
            },
        )
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }

    pub fn split(self) -> (ErrorKind, Span) {
        (*self.kind, self.span)
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
            Self::InvalidAssignmentTarget => write!(f, "Invalid assignment target"),
            Self::TypeMismatch {
                expected,
                found,
                reason,
            } => match reason {
                TypeMismatchReason::FunctionReturn => write!(
                    f,
                    "Type mismatch, expected return type '{}', found '{}'",
                    expected, found
                ),
                TypeMismatchReason::FunctionArgument => write!(
                    f,
                    "Type mismatch, expected argument type '{}', found '{}'",
                    expected, found
                ),
                TypeMismatchReason::BinaryOperation(_) => write!(
                    f,
                    "Type mismatch, expected both operands to be of type '{}', found '{}'",
                    expected, found
                ),
                TypeMismatchReason::VariableAssignment => write!(
                    f,
                    "Type mismatch, expected variable to be of type '{}', found '{}'",
                    expected, found
                ),
            },
            Self::TypeNotFound (name) => write!(f, "Type '{}' was not found", name),
            Self::StructFieldNotInitialized(name) => write!(f, "Field '{}' was not initialized", name),
            Self::StructFieldNotFound(name) => write!(f, "Field '{}' was not found in struct", name),
            Self::FailedToAccessField(type_def) => match type_def {
                TypeDef::PrimitiveInt | 
                TypeDef::PrimitiveFloat |
                TypeDef::PrimitiveString |
                TypeDef::PrimitiveBool => write!(f, "Primitive values do not have fields"),

                TypeDef::Void => write!(f, "Void does not have fields"),
                TypeDef::Struct(_) => write!(f, "Failed to access field of struct"),
            },
            Self::NoMainFunction => write!(f, "No main function found"),
            Self::Break | Self::Continue | Self::Return(_) => unreachable!("Break, Continue and Return should be handled by the executor. They should never result in an error"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        std::error::Error::source(&self.kind)
    }
}

impl From<ErrorKind> for Error {
    fn from(value: ErrorKind) -> Self {
        Error {
            kind: Box::new(value),
            span: Span::default(),
        }
    }
}
