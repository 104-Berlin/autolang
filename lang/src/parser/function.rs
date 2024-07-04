use crate::error::Spanned;

use super::{expression::Expr, type_def::TypeID};

pub type ArgumentDecl = (Spanned<String>, Spanned<TypeID>);

#[derive(Debug, Clone)]
pub struct FunctionProto {
    pub name: Spanned<String>,
    pub arguments: Spanned<Vec<ArgumentDecl>>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub proto: Spanned<FunctionProto>,
    pub body: Spanned<Expr>,
}
