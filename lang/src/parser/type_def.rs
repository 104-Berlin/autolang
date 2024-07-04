use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Pointer(Box<Type>),
    Array(Box<Type>),
    Struct(Vec<StructField>),
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: TypeID,
}

#[derive(Debug, Clone)]
pub enum TypeID {
    Int,
    Float,
    String,
    Bool,

    User(String),
}

impl Display for TypeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeID::Int => write!(f, "int"),
            TypeID::Float => write!(f, "float"),
            TypeID::String => write!(f, "string"),
            TypeID::Bool => write!(f, "bool"),
            TypeID::User(name) => write!(f, "{}", name),
        }
    }
}
