use core::fmt;
use std::fmt::Formatter;

/// An attribute applied to its owner
#[derive(Debug, Clone, Default)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}
impl Attribute {
    pub fn new(name: &str) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: None,
        }
    }

    pub fn is_valid(&self) -> bool {
        self.name.is_empty()
    }
}
impl std::fmt::Display for Attribute {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result {
        match write!(fmt, "#[{}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if self.value.is_some() {
            match write!(fmt, "({})", self.value.as_ref().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match writeln!(fmt, "]") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
