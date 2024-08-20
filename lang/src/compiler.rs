use source_span::Span;
use virtual_machine::{
    instruction::writer::InstructionWriter,
    program_builder::{Buildable, ProgramBuilder},
};

use crate::{
    error::{ALError, ALResult},
    module::Module,
    spanned::Spanned,
};

pub struct Context;

impl Context {
    pub fn new() -> Self {
        Context
    }
}

pub struct Compiler {
    context: Context,
}

impl Default for Compiler {
    fn default() -> Self {
        Compiler {
            context: Context::new(),
        }
    }
}

impl Compiler {
    pub fn compile(&mut self, module: &Module) -> Result<[u32; 1024], ALError> {
        let mut builder = ProgramBuilder::new();
        module.build(&mut builder)?;
        Ok(builder.finish())
    }
}
