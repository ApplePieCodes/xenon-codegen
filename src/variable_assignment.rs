use core::fmt;

use crate::{expression::Expression, identifier::IdentifierAccess};

#[derive(Debug, Clone, Default)]
pub struct VariableAssignment {
    pub name: IdentifierAccess,
    pub operator: String,
    pub value: Expression,
}
impl VariableAssignment {
    pub fn new(name: IdentifierAccess, op: String, value: Expression) -> VariableAssignment {
        return VariableAssignment {
            name: name,
            operator: op,
            value: value,
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }
        if !["=", "+=", "-=", "*=", "/="].contains(&self.operator.as_str()) {
            return false;
        }
        if !self.value.is_valid() {
            return false;
        }
        return true;
    }
}
impl fmt::Display for VariableAssignment {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{} {} {}", self.name, self.operator, self.value) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
