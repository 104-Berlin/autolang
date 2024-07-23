use crate::{
    parser::{function::FunctionDecl, structs::Struct},
    spanned::Spanned,
};

pub struct Module {
    name: String,
    functions: Vec<Spanned<FunctionDecl>>,
    structs: Vec<Spanned<Struct>>,
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

    pub fn add_struct(&mut self, strct: Spanned<Struct>) {
        self.structs.push(strct);
    }

    pub fn structs(&self) -> &[Spanned<Struct>] {
        &self.structs
    }
}
