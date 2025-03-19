use core::fmt;

use crate::statement::Statement;

/// Defines a scope
#[derive(Debug, Clone, Default)]
pub struct Scope {
    pub items: Vec<Statement>,
}
impl Scope {
    pub fn new() -> Scope {
        Scope { items: vec![] }
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.items.len() {
            if !self.items[i].is_valid() {
                return false;
            }
        }

        true
    }
}
impl fmt::Display for Scope {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match writeln!(fmt, "{{") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for i in 0..self.items.len() {
            match write!(fmt, "{}", self.items[i]) {
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
