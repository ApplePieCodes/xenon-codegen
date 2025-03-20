use std::fmt;

use crate::{Visibility, attribute::Attribute};

/// Represents a declaration of an Enum type
#[derive(Debug, Clone, Default)]
pub struct Enum {
    /// Any Attributes applied to the enum
    pub attrs: Vec<Attribute>,
    /// The Visibility of the enum
    pub visibility: Visibility,
    /// The name of the enum
    pub name: String,
    /// The variants of the enum
    pub variants: Vec<Variant>,
}
impl Enum {
    /// Creates a valid instance of Enum
    pub fn new(name: String) -> Enum {
        Enum {
            attrs: vec![],
            visibility: Visibility::Private,
            name,
            variants: vec![],
        }
    }

    /// Checks if the Enum is valid
    pub fn is_valid(&self) -> bool {
        // Check all attrs
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        // Name must not be empty
        if self.name.is_empty() {
            return false;
        }
        // Check all variants
        for i in 0..self.variants.len() {
            if !self.variants[i].is_valid() {
                return false;
            }
        }
        true
    }
}
impl fmt::Display for Enum {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print every attribute
        for i in 0..self.attrs.len() {
            match write!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        // Write the visibility, enum, the name, and start the scope for variants
        match write!(fmt, "{} enum {} {{", self.visibility, self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        // Write every variant
        for i in 0..self.variants.len() {
            match writeln!(fmt, "{}", self.variants[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        // Close the scope
        match write!(fmt, "}}") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

/// Represents an enum variant
#[derive(Debug, Clone, Default)]
pub struct Variant {
    /// Any attributes for the variant
    pub attrs: Vec<Attribute>,
    /// The name of the variant
    pub name: String,
}
impl Variant {
    /// Creates a valid instance of Variant
    pub fn new(name: &str) -> Variant {
        Variant {
            attrs: vec![],
            name: name.to_string(),
        }
    }

    /// Checks if the Variant is valid
    pub fn is_valid(&self) -> bool {
        // Check every attribute
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        // Name must not be empty
        if self.name.is_empty() {
            return false;
        }
        true
    }
}
impl fmt::Display for Variant {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print every attribute
        for i in 0..self.attrs.len() {
            match write!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        // And then print the name
        match writeln!(fmt, "{},", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
