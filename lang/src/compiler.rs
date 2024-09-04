use context::Context;

use crate::module::Module;
use miette::miette;

pub mod context;
pub mod scope;
pub mod unresolved_instruction;

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
        module.build(&mut self.context)?;
        self.context.finish().map_err(|e| miette!("{e}"))
    }
}
