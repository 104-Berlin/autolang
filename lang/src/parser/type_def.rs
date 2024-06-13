pub enum Type {
    Int,
    Float,
    String,
    Bool,
    Pointer(Box<Type>),
    Array(Box<Type>),
    Struct(Vec<StructField>),
}

pub struct StructField {
    pub name: String,
    pub ty: Type,
}
