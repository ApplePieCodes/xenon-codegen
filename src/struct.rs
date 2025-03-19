use core::fmt;

use crate::{
    Visibility, attribute::Attribute, r#type::Type, variable_definition::VariableDefinition,
};

#[derive(Debug, Clone, Default)]
pub struct Struct {
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Type,
    pub base: Option<Type>,
    pub properties: Vec<VariableDefinition>,
}
impl Struct {
    pub fn new(name: Type) -> Struct {
        return Struct {
            attrs: vec![],
            visibility: Visibility::Private,
            name: name,
            base: None,
            properties: vec![],
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
        match self.base.clone() {
            Some(b) => {
                if !b.is_valid() {
                    return false;
                }
            }
            None => (),
        }
        for i in 0..self.properties.len() {
            if !self.properties[i].is_valid() {
                return false;
            }
        }

        return true;
    }
}
impl fmt::Display for Struct {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.attrs.len() {
            match write!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match self.base.clone() {
            Some(b) => match write!(fmt, "{} struct {} : {} {{\n", self.visibility, self.name, b) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => match write!(fmt, "{} struct {} {{\n", self.visibility, self.name) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
        }
        for i in 0..self.properties.len() {
            match write!(fmt, "{};\n", self.properties[i]) {
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
