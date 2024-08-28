pub mod compiler;
use spanned::Spanned;

pub mod error;
pub mod execution;
pub mod input_stream;
pub mod module;
pub mod parser;
pub mod prelude;
pub mod spanned;
pub mod system_functions;
pub mod tokenizer;

pub type ALResult<T> = Result<Spanned<T>, miette::Error>;
