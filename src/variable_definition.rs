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
        VariableDefinition {
            visibility: Visibility::Private,
            name,
            ty: None,
            value: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }
        if let Some(t) = self.ty.clone() {
            if !t.is_valid() {
                return false;
            }
        }
        if let Some(v) = self.value.clone() {
            if !v.is_valid() {
                return false;
            }
        }

        true
    }
}
impl fmt::Display for VariableDefinition {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "let {}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if let Some(t) = self.ty.clone() {
            match write!(fmt, ": {}", t) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        if let Some(v) = self.value.clone() {
            match write!(fmt, " = {}", v) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }

        Ok(())
    }
}
