use core::fmt;

use crate::{attribute::Attribute, function::Function, r#type::Type};

#[derive(Debug, Clone, Default)]
pub struct Impl {
    pub attrs: Vec<Attribute>,
    pub target: Type,
    /// The trait the impl is implimenting
    pub r#trait: Option<Type>,
    pub methods: Vec<Function>,
}
impl Impl {
    pub fn new(target: Type) -> Impl {
        return Impl {
            attrs: vec![],
            target: target,
            r#trait: None,
            methods: vec![],
        };
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        if !self.target.is_valid() {
            return false;
        }
        match self.r#trait.clone() {
            Some(t) => {
                if !t.is_valid() {
                    return false;
                }
            }
            None => (),
        }
        return true;
    }
}
impl fmt::Display for Impl {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.attrs.len() {
            match write!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match self.r#trait.clone() {
            Some(t) => match write!(fmt, "impliment {} for {} {{\n", t, self.target) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => match write!(fmt, "impliment {} {{\n", self.target) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
        }
        for i in 0..self.methods.len() {
            match write!(fmt, "{}\n", self.methods[i]) {
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
