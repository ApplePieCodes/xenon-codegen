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
        Impl {
            attrs: vec![],
            target,
            r#trait: None,
            methods: vec![],
        }
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
        if let Some(t) = self.r#trait.clone() {
            if !t.is_valid() {
                return false;
            }
        }
        true
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
            Some(t) => match writeln!(fmt, "impliment {} for {} {{", t, self.target) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => match writeln!(fmt, "impliment {} {{", self.target) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
        }
        for i in 0..self.methods.len() {
            match writeln!(fmt, "{}", self.methods[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "}}") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
