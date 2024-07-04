use crate::parser::function::FunctionDecl;

pub struct Module {
    name: String,
    functions: Vec<FunctionDecl>,
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

    pub fn add_function(&mut self, func: FunctionDecl) {
        self.functions.push(func);
    }

    pub fn functions(&self) -> &[FunctionDecl] {
        &self.functions
    }
}
