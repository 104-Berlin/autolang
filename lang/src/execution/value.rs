use std::{
    any::Any,
    fmt::{Debug, Display},
};

use crate::{
    error::{ALError, ALResult, ErrorKind, TypeMismatchReason},
    parser::{binary_expression::BinaryOperator, structs::StructValue, type_def::TypeID},
    spanned::Spanned,
};

pub struct Value {
    pub value: Box<dyn Any>,
    pub type_id: TypeID,
}

impl Value {
    pub fn new_void() -> Self {
        Self {
            value: Box::new(()),
            type_id: TypeID::Void,
        }
    }

    pub fn new_int(value: i64) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeID::Int,
        }
    }

    pub fn new_float(value: f64) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeID::Float,
        }
    }

    pub fn new_bool(value: bool) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeID::Bool,
        }
    }

    pub fn new_string(value: String) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeID::String,
        }
    }

    pub fn new_struct(name: String, value: StructValue) -> Self {
        Self {
            value: Box::new(value),
            type_id: TypeID::User(name),
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if self.type_id == TypeID::Int {
            self.value.downcast_ref::<i64>().cloned()
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if self.type_id == TypeID::Float {
            self.value.downcast_ref::<f64>().cloned()
        } else {
            None
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        if self.type_id == TypeID::Bool {
            self.value.downcast_ref::<bool>().cloned()
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if self.type_id == TypeID::String {
            self.value.downcast_ref::<String>().map(|s| s.as_str())
        } else {
            None
        }
    }

    pub fn as_struct(&self) -> Option<&StructValue> {
        if matches!(self.type_id, TypeID::User(_)) {
            self.value.downcast_ref::<StructValue>()
        } else {
            None
        }
    }

    pub fn set_value(&mut self, other: &Spanned<Self>) -> ALResult<()> {
        if self.type_id == other.value.type_id {
            match self.type_id {
                TypeID::Int => self.value = Box::new(other.value.as_int().unwrap()),
                TypeID::Float => self.value = Box::new(other.value.as_float().unwrap()),
                TypeID::String => {
                    self.value = Box::new(other.value.as_string().unwrap().to_string())
                }
                TypeID::Bool => self.value = Box::new(other.value.as_bool().unwrap()),
                TypeID::Void => {}
                TypeID::User(_) => todo!("Assign user defined values"),
            }
            Ok(Spanned::new((), other.span))
        } else {
            Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::VariableAssignment,
            ))
        }
    }

    pub fn add(&self, other: &Spanned<Self>) -> ALResult<Self> {
        /*if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Add),
            ));
        }*/

        match (&self.type_id, &other.value.type_id) {
            (TypeID::Int, TypeID::Int) => Ok(Self::new_int(
                self.as_int().unwrap() + other.value.as_int().unwrap(),
            )),
            // Enable for implicit casting
            // (TypeID::Int, TypeID::Float) => Ok(Self::new_float(
            //     self.as_int().unwrap() as f64 + other.value.as_float().unwrap(),
            // )),
            (TypeID::Float, TypeID::Float) => Ok(Self::new_float(
                self.as_float().unwrap() + other.value.as_float().unwrap(),
            )),
            // Enable for implicit casting
            // (TypeID::Float, TypeID::Int) => Ok(Self::new_float(
            //     self.as_float().unwrap() + other.value.as_int().unwrap() as f64,
            // )),
            (TypeID::String, TypeID::String) => {
                let mut s = self.as_string().unwrap().to_string();
                s.push_str(other.value.as_string().unwrap());
                Ok(Self::new_string(s))
            }
            (TypeID::String, TypeID::Int)
            | (TypeID::String, TypeID::Float)
            | (TypeID::String, TypeID::Bool) => Ok(Self::new_string(format!(
                "{}{}",
                self.as_string().unwrap(),
                other.value
            ))),
            (TypeID::Int, TypeID::String)
            | (TypeID::Float, TypeID::String)
            | (TypeID::Bool, TypeID::String) => Ok(Self::new_string(format!(
                "{}{}",
                self,
                other.value.as_string().unwrap()
            ))),
            (TypeID::Bool, _) => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            (_, TypeID::Bool) => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            (TypeID::Void, _) => Ok(Value::new_void()),
            (_, TypeID::Void) => Ok(self.clone()),
            (TypeID::User(_), _) => todo!(),
            (_, TypeID::User(_)) => todo!(),
            (_, _) => Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Add),
            )),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn sub(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Substract),
            ));
        }

        match self.type_id {
            TypeID::Int => Ok(Self::new_int(
                self.as_int().unwrap() - other.value.as_int().unwrap(),
            )),
            TypeID::Float => Ok(Self::new_float(
                self.as_float().unwrap() - other.value.as_float().unwrap(),
            )),
            TypeID::String => todo!(),
            TypeID::Bool => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            TypeID::Void => todo!(),
            TypeID::User(_) => todo!(),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn mul(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Multiply),
            ));
        }

        match self.type_id {
            TypeID::Int => Ok(Self::new_int(
                self.as_int().unwrap() * other.value.as_int().unwrap(),
            )),
            TypeID::Float => Ok(Self::new_float(
                self.as_float().unwrap() * other.value.as_float().unwrap(),
            )),
            TypeID::String => todo!(),
            TypeID::Bool => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            TypeID::Void => todo!(),
            TypeID::User(_) => todo!(),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn div(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Divide),
            ));
        }

        match self.type_id {
            TypeID::Int => Ok(Self::new_int(
                self.as_int().unwrap() / other.value.as_int().unwrap(),
            )),
            TypeID::Float => Ok(Self::new_float(
                self.as_float().unwrap() / other.value.as_float().unwrap(),
            )),
            TypeID::String => todo!(),
            TypeID::Bool => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            TypeID::Void => todo!(),
            TypeID::User(_) => todo!(),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    // Logical operations
    pub fn and(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != TypeID::Bool || other.value.type_id != TypeID::Bool {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::And),
            ));
        }

        Ok(Self::new_bool(
            self.as_bool().unwrap() && other.value.as_bool().unwrap(),
        ))
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn or(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != TypeID::Bool || other.value.type_id != TypeID::Bool {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Or),
            ));
        }

        Ok(Self::new_bool(
            self.as_bool().unwrap() || other.value.as_bool().unwrap(),
        ))
        .map(|v| Spanned::new(v, other.span))
    }

    // Comparison operations
    /// Equal function. Trys to compare two values and returns a boolean value.
    /// ### NOTE
    /// This will always return a boolean value or an error if the types dont match.
    pub fn eq(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::Equal),
            ));
        }

        match self.type_id {
            TypeID::Int => Ok(Self::new_bool(
                self.as_int().unwrap() == other.value.as_int().unwrap(),
            )),
            TypeID::Float => Ok(Self::new_bool(
                self.as_float().unwrap() == other.value.as_float().unwrap(),
            )),
            TypeID::String => Ok(Self::new_bool(
                self.as_string().unwrap() == other.value.as_string().unwrap(),
            )),
            TypeID::Bool => Ok(Self::new_bool(
                self.as_bool().unwrap() == other.value.as_bool().unwrap(),
            )),
            TypeID::Void => Ok(Self::new_bool(true)),
            TypeID::User(_) => todo!(),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn neq(&self, other: &Spanned<Self>) -> ALResult<Self> {
        self.eq(other)
            // Value will be a bool
            .map(|v| Self::new_bool(!v.value.as_bool().unwrap()))
            .map(|v| Spanned::new(v, other.span))
    }

    pub fn lt(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::LessThan),
            ));
        }

        match self.type_id {
            TypeID::Int => Ok(Self::new_bool(
                self.as_int().unwrap() < other.value.as_int().unwrap(),
            )),
            TypeID::Float => Ok(Self::new_bool(
                self.as_float().unwrap() < other.value.as_float().unwrap(),
            )),
            TypeID::String => Ok(Self::new_bool(
                self.as_string().unwrap() < other.value.as_string().unwrap(),
            )),
            TypeID::Bool => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            TypeID::Void => Ok(Self::new_bool(true)),
            TypeID::User(_) => todo!(),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn gt(&self, other: &Spanned<Self>) -> ALResult<Self> {
        if self.type_id != other.value.type_id {
            return Err(ALError::new_type_mismatch(
                other.span,
                self.type_id.clone(),
                other.value.type_id.clone(),
                TypeMismatchReason::BinaryOperation(BinaryOperator::GreaterThan),
            ));
        }

        match self.type_id {
            TypeID::Int => Ok(Self::new_bool(
                self.as_int().unwrap() > other.value.as_int().unwrap(),
            )),
            TypeID::Float => Ok(Self::new_bool(
                self.as_float().unwrap() > other.value.as_float().unwrap(),
            )),
            TypeID::String => Ok(Self::new_bool(
                self.as_string().unwrap() > other.value.as_string().unwrap(),
            )),
            TypeID::Bool => Err(ALError::new(other.span, ErrorKind::InvalidOperator)),
            TypeID::Void => Ok(Self::new_bool(true)),
            TypeID::User(_) => todo!(),
        }
        .map(|v| Spanned::new(v, other.span))
    }

    pub fn lte(&self, other: &Spanned<Self>) -> ALResult<Self> {
        self.lt(other)
            // Value will be a bool
            .map(|v| {
                Self::new_bool(
                    v.value.as_bool().unwrap() || self.eq(other).unwrap().value.as_bool().unwrap(),
                )
            })
            .map(|v| Spanned::new(v, other.span))
    }

    pub fn gte(&self, other: &Spanned<Self>) -> ALResult<Self> {
        self.gt(other)
            // Value will be a bool
            .map(|v| {
                Self::new_bool(
                    v.value.as_bool().unwrap() || self.eq(other).unwrap().value.as_bool().unwrap(),
                )
            })
            .map(|v| Spanned::new(v, other.span))
    }

    pub fn from_generic<T>(value: T) -> Self
    where
        T: Into<Self>,
    {
        value.into()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        match &self.type_id {
            TypeID::Int => Self::new_int(self.as_int().unwrap()),
            TypeID::Float => Self::new_float(self.as_float().unwrap()),
            TypeID::String => Self::new_string(self.as_string().unwrap().to_string()),
            TypeID::Bool => Self::new_bool(self.as_bool().unwrap()),
            TypeID::Void => Self::new_void(),
            TypeID::User(name) => Self::new_struct(name.clone(), self.as_struct().unwrap().clone()),
        }
    }
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} ({})", self.value, self.type_id)
    }
}

impl From<TypeID> for Value {
    fn from(value: TypeID) -> Self {
        match value {
            TypeID::Int => Self::new_int(0),
            TypeID::Float => Self::new_float(0.0),
            TypeID::String => Self::new_string(String::new()),
            TypeID::Bool => Self::new_bool(false),
            TypeID::Void => Self::new_void(),
            TypeID::User(_) => todo!(),
        }
    }
}

impl From<Spanned<TypeID>> for Value {
    fn from(value: Spanned<TypeID>) -> Self {
        Self::from(value.value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Self::new_int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Self::new_float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::new_bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::new_string(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::new_string(value.to_string())
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Self::new_void()
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.type_id {
            TypeID::Int => write!(f, "{}", self.as_int().unwrap()),
            TypeID::Float => write!(f, "{}", self.as_float().unwrap()),
            TypeID::String => write!(f, "{}", self.as_string().unwrap()),
            TypeID::Bool => write!(f, "{}", self.as_bool().unwrap()),
            TypeID::Void => write!(f, "void"),
            TypeID::User(_) => todo!(),
        }
    }
}
