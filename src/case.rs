use crate::{expression::Expression, scope::Scope};

#[derive(Debug, Clone, Default)]
pub struct Case {
    pub condition: Expression,
    pub body: Scope,
}
impl Case {
    pub fn new(condition: Expression, body: Scope) -> Case {
        return Case {
            condition: condition,
            body: body,
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.condition.is_valid() && self.body.is_valid();
    }
}
impl std::fmt::Display for Case {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(
            fmt,
            "case {} {}",
            self.condition.to_string(),
            self.body.to_string()
        ) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }

        return Ok(());
    }
}
