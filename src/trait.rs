use core::fmt;
use std::vec;

use crate::{
    Visibility, attribute::Attribute, function::Function, r#type::Type,
    variable_definition::VariableDefinition,
};

#[derive(Debug, Clone, Default)]
pub struct Trait {
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Type,
    pub fields: Vec<VariableDefinition>,
    pub methods: Vec<Function>,
}
impl Trait {
    pub fn new(name: Type) -> Trait {
        return Trait {
            attrs: vec![],
            visibility: Visibility::Private,
            name: name,
            fields: vec![],
            methods: vec![],
        };
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        if !self.name.is_valid() {
            return false;
        }
        for i in 0..self.fields.len() {
            if !self.fields[i].is_valid() {
                return false;
            }
        }
        for i in 0..self.methods.len() {
            if !self.methods[i].is_valid() {
                return false;
            }
        }
        return true;
    }
}
impl fmt::Display for Trait {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.attrs.len() {
            match write!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "{} trait {} {{\n", self.visibility, self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for i in 0..self.fields.len() {
            match write!(fmt, "{}", self.fields[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        for i in 0..self.methods.len() {
            match write!(fmt, "{}", self.methods[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "}}") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
