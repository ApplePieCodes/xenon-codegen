use crate::{expression::Expression, scope::Scope};

/// Represents a case in a switch statement
#[derive(Debug, Clone, Default)]
pub struct Case {
    /// The condition of the case
    pub condition: Expression,
    /// The body to be executed if true
    pub body: Scope,
}
impl Case {
    /// Creates a valid instance of Case
    pub fn new(condition: Expression, body: Scope) -> Case {
        Case { condition, body }
    }

    /// Check if the case is valid
    pub fn is_valid(&self) -> bool {
        // Very simple. just check all elements of self
        return self.condition.is_valid() && self.body.is_valid();
    }
}
impl std::fmt::Display for Case {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Also very simple. Just print the case.
        match write!(fmt, "case {} {}", self.condition, self.body) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
