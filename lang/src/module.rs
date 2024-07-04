use crate::{error::Spanned, parser::function::FunctionDecl};

pub struct Module {
    name: String,
    functions: Vec<Spanned<FunctionDecl>>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::default(),
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
}
