use virtual_machine::program_builder::{Buildable, ProgramBuilder};

use crate::module::Module;
use miette::miette;

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
    pub fn compile(&mut self, module: &Module) -> Result<[u32; 1024], miette::Error> {
        let mut builder = ProgramBuilder::default();
        module.build(&mut builder)?;
        builder.finish().map_err(|e| miette!("{e}"))
    }
}
