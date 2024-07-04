use super::type_def::TypeID;

pub type ArgumentDecl = (String, TypeID);

#[derive(Debug, Clone)]
pub struct FunctionProto {
    pub name: String,
    pub arguments: Vec<ArgumentDecl>,
}

#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub proto: FunctionProto,
    // pub body: Body
}
