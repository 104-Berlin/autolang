use crate::{execution::value::Value, spanned::Spanned};

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

/// I want to make this as small as possible. The order of the fields is very importent here
#[derive(Default, Clone)]
pub struct StructValue {
    fields: Vec<Spanned<Value>>,
}

impl StructValue {
    pub fn push_field(&mut self, value: Spanned<Value>) {
        self.fields.push(value);
    }

    pub fn get_field(&self, index: usize) -> Option<&Spanned<Value>> {
        self.fields.get(index)
    }
}
