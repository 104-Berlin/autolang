use std::fmt::Display;

use virtual_machine::{instruction::Instruction, register::Register};

use crate::{
    compiler::compiler_context::{Buildable, CompilerContext},
    spanned::{Spanned, WithSpan},
    ALResult,
};

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

impl Buildable for Spanned<FunctionDecl> {
    fn build(&self, builder: &mut CompilerContext) -> ALResult<()> {
        builder.build_instruction(Instruction::Push(Register::BP.into()).with_span(self.span))?;

        builder.build_instruction(
            Instruction::Move {
                // SP -> BP
                src: Register::SP.into(),
                dst: Register::BP.into(),
            }
            .with_span(self.span),
        )?;

        self.body.build(builder)?;

        builder.build_instruction(Instruction::Pop(Register::BP.into()).with_span(self.span))
    }
}
