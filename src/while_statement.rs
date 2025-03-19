use core::fmt;

use crate::{expression::Expression, statement::Statement};

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
}
impl WhileStatement {
    pub fn new(condition: Expression, body: Statement) -> WhileStatement {
        return WhileStatement {
            condition: condition,
            body: Box::new(body),
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.condition.is_valid() {
            return false;
        }
        if !self.body.is_valid() {
            return false;
        }
        return true;
    }
}
impl fmt::Display for WhileStatement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "while ({}) {}", self.condition, self.body) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
