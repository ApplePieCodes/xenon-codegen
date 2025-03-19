use crate::{expression::Expression, scope::Scope};

#[derive(Debug, Clone, Default)]
pub struct Case {
    pub condition: Expression,
    pub body: Scope,
}
impl Case {
    pub fn new(condition: Expression, body: Scope) -> Case {
        Case { condition, body }
    }

    pub fn is_valid(&self) -> bool {
        self.condition.is_valid() && self.body.is_valid()
    }
}
impl std::fmt::Display for Case {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "case {} {}", self.condition, self.body) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        Ok(())
    }
}
