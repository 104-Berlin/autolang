use std::fmt::Display;

use virtual_machine::program_builder::{Buildable, ProgramBuilder};

use crate::{error::ALError, spanned::Spanned};

use super::{expression::Expr, type_def::TypeID};

pub type ArgumentDecl = (Spanned<String>, Spanned<TypeID>);

#[derive(Debug, Clone)]
pub struct FunctionProto {
    pub name: Spanned<String>,
    pub return_type: Spanned<TypeID>,
    pub arguments: Spanned<Vec<ArgumentDecl>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub proto: Spanned<FunctionProto>,
    pub body: Spanned<Expr>,
}

impl Display for FunctionProto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fn {}(", self.name.value)?;
        for (i, arg) in self.arguments.value.iter().enumerate() {
            if i != 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", arg.0.value, arg.1.value)?;
        }
        write!(f, ") -> {}", self.return_type.value)
    }
}

impl Display for FunctionDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {{\n{}\n}}", self.proto.value, self.body.value)
    }
}

impl Buildable for FunctionDecl {
    type Error = ALError;
    fn build(&self, builder: &mut ProgramBuilder) -> Result<(), Self::Error> {
        self.body.build(builder)
    }
}
