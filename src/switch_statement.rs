use core::fmt;

use crate::{case::Case, expression::Expression};

#[derive(Debug, Clone, Default)]
pub struct SwitchStatement {
    pub condition: Expression,
    pub cases: Vec<Case>,
}
impl SwitchStatement {
    pub fn new(condition: Expression) -> SwitchStatement {
        return SwitchStatement {
            condition: condition,
            cases: vec![],
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.condition.is_valid() {
            return false;
        }

        for i in 0..self.cases.len() {
            if !self.cases[i].is_valid() {
                return false;
            }
        }
        return true;
    }
}
impl fmt::Display for SwitchStatement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "switch ({}) {{\n", self.condition) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for i in 0..self.cases.len() {
            match write!(fmt, "{}", self.cases[i]) {
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
