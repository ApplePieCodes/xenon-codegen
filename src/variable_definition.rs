use core::fmt;

use crate::{Visibility, expression::Expression, r#type::Type};

#[derive(Debug, Clone)]
pub struct VariableDefinition {
    pub visibility: Visibility,
    pub name: String,
    pub ty: Option<Type>,
    pub value: Option<Expression>,
}
impl VariableDefinition {
    pub fn new(name: String) -> VariableDefinition {
        return VariableDefinition {
            visibility: Visibility::Private,
            name: name,
            ty: None,
            value: None,
        };
    }

    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }
        match self.ty.clone() {
            Some(t) => {
                if !t.is_valid() {
                    return false;
                }
            }
            None => (),
        }
        match self.value.clone() {
            Some(v) => {
                if !v.is_valid() {
                    return false;
                }
            }
            None => (),
        }

        return true;
    }
}
impl fmt::Display for VariableDefinition {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "let {}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.ty.clone() {
            Some(t) => match write!(fmt, ": {}", t) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => (),
        }
        match self.value.clone() {
            Some(v) => match write!(fmt, " = {}", v) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => (),
        }

        return Ok(());
    }
}
