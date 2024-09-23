use crate::spanned::Spanned;

use super::type_def::TypeID;

#[derive(Debug, Clone)]
pub struct Struct {
    pub fields: Vec<Spanned<(String, TypeID)>>,
}

impl Struct {
    pub fn new_unit() -> Self {
        Self {
            fields: Vec::default(),
        }
    }

    pub fn new(fields: Vec<Spanned<(String, TypeID)>>) -> Self {
        Self { fields }
    }
}
