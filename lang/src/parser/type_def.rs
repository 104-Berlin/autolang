use std::fmt::Display;

use crate::prelude::FunctionDecl;

use super::structs::Struct;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeID {
    Int,
    Float,
    String,
    Bool,

    Void,

    User(String),
}

impl TypeID {
    pub fn from_string(s: &str) -> Self {
        match s {
            "int" => TypeID::Int,
            "float" => TypeID::Float,
            "String" => TypeID::String,
            "bool" => TypeID::Bool,
            "void" => TypeID::Void,
            _ => TypeID::User(s.to_string()),
        }
    }
}

impl Display for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeID::Int => write!(f, "int"),
            TypeID::Float => write!(f, "float"),
            TypeID::String => write!(f, "string"),
            TypeID::Bool => write!(f, "bool"),
            TypeID::Void => write!(f, "void"),
            TypeID::User(name) => write!(f, "{}", name),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TypeDef {
    PrimitiveInt,
    PrimitiveFloat,
    PrimitiveString,
    PrimitiveBool,

    Void,

    Struct(Struct),
}
