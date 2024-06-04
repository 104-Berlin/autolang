pub struct Module {
    name: String,
    functions: Vec<FunctionDefinition>,
}

pub struct FunctionDefinition {
    pub name: String,
    //arguments: Vec<String>,
    // body: Option<Body>
}

impl Module {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            functions: Vec::default(),
        }
    }

    pub fn add_function(&mut self, func: FunctionDefinition) {
        self.functions.push(func);
    }

    pub fn functions(&self) -> &[FunctionDefinition] {
        &self.functions
    }
}
