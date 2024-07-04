use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TypeID {
    Int,
    Float,
    String,
    Bool,

    Void,

    User(String),
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
