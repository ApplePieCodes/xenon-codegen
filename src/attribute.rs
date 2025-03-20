use core::fmt;
use std::fmt::Formatter;

/// An attribute applied to its owner. Usually metadata for the compiler.
#[derive(Debug, Clone, Default)]
pub struct Attribute {
    /// The name of the attribute
    pub name: String,
    /// The value, if any
    pub value: Option<String>,
}
impl Attribute {
    /// Create a new valid attribute
    pub fn new(name: &str) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: None,
        }
    }

    /// Check if the attribute is valid
    pub fn is_valid(&self) -> bool {
        // Make sure that all present fields are not empty
        if self.name.is_empty() {
            return false;
        }
        match self.value.clone() {
            Some(v) => {
                if v.is_empty() {
                    return false;
                }
            }
            None => ()
        }

        return true;
    }
}
impl std::fmt::Display for Attribute {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        // Print the start of the tag and the name
        match write!(fmt, "#[{}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        // If there's a value, print it in parentheses
        if self.value.is_some() {
            match write!(fmt, "({})", self.value.as_ref().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        // End the tag
        match writeln!(fmt, "]") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
