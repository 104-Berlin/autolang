use miette::SourceSpan;

use crate::{
    compiler::compiler_context::{Buildable, CompilerContext},
    parser::{function::FunctionDecl, structs::Struct},
    spanned::{SpanExt, Spanned, WithSpan},
    ALResult,
};

pub struct Module {
    name: String,
    functions: Vec<Spanned<FunctionDecl>>,
    structs: Vec<(Spanned<String>, Spanned<Struct>)>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::default(),
            structs: Vec::default(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_function(&mut self, func: Spanned<FunctionDecl>) {
        self.functions.push(func);
    }

    pub fn functions(&self) -> &[Spanned<FunctionDecl>] {
        &self.functions
    }

    pub fn add_struct(&mut self, name: Spanned<String>, strct: Spanned<Struct>) {
        self.structs.push((name, strct));
    }

    pub fn structs(&self) -> &[(Spanned<String>, Spanned<Struct>)] {
        &self.structs
    }
}

impl Buildable for Module {
    fn build(&self, builder: &mut CompilerContext) -> ALResult<()> {
        let mut span: Option<SourceSpan> = None;

        for func in self.functions() {
            func.build(builder)?;
            match span {
                Some(s) => {
                    span = Some(s.union(&func.span));
                }
                None => {
                    span = Some(func.span);
                }
            }
        }

        Ok(().with_span(span.expect("Empty module")))
    }
}
