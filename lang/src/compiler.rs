use compiler_context::{Buildable, CompilerContext};

use crate::module::Module;

pub mod compiler_context;
pub mod scope;
pub mod unresolved_instruction;

#[derive(Default)]
pub struct Compiler {
    context: CompilerContext,
}

impl Compiler {
    pub fn compile(mut self, module: &Module) -> Result<([u32; 1024], u32), miette::Error> {
        module.build(&mut self.context)?;

        self.context.finish().map(|m| *m)
    }
}
