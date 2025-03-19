use std::{fmt, string};

use crate::{Visibility, attribute::Attribute, r#type::Type};

#[derive(Debug, Clone, Default)]
pub struct Enum {
    pub visibility: Visibility,
    pub attrs: Vec<Attribute>,
    pub name: String,
    pub variants: Vec<Variant>,
}
impl Enum {
    pub fn new(name: String) -> Enum {
        return Enum {
            visibility: Visibility::Private,
            attrs: vec![],
            name: name,
            variants: vec![],
        };
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        if self.name.is_empty() {
            return false;
        }
        for i in 0..self.variants.len() {
            if !self.variants[i].is_valid() {
                return false;
            }
        }
        return true;
    }
}
impl fmt::Display for Enum {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.attrs.len() {
            match writeln!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "{} enum {} {{", self.visibility, self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for i in 0..self.variants.len() {
            match writeln!(fmt, "{}", self.variants[i]) {
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

#[derive(Debug, Clone, Default)]
pub struct Variant {
    pub attrs: Vec<Attribute>,
    pub name: String,
}
impl Variant {
    pub fn new(name: String) -> Variant {
        return Variant {
            attrs: vec![],
            name: name,
        };
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        return true;
    }
}
impl fmt::Display for Variant {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.attrs.len() {
            match writeln!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match writeln!(fmt, "{},", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
