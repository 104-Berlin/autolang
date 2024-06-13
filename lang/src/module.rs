use crate::parser::function::FunctionDeclaration;

pub struct Module {
    name: String,
    functions: Vec<FunctionDeclaration>,
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::default(),
        }
    }

    pub fn add_function(&mut self, func: FunctionDeclaration) {
        self.functions.push(func);
    }

    pub fn functions(&self) -> &[FunctionDeclaration] {
        &self.functions
    }
}
