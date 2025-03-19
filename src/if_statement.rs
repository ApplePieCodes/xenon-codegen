use core::fmt;

use crate::{expression::Expression, statement::Statement};

#[derive(Debug, Clone, Default)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
}
impl IfStatement {
    pub fn new(condition: Expression, body: Statement) -> IfStatement {
        return IfStatement {
            condition: condition,
            body: Box::new(body),
            else_body: None,
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.condition.is_valid() {
            return false;
        }
        if !self.body.is_valid() {
            return false;
        }
        match self.else_body.clone() {
            Some(b) => {
                if !b.is_valid() {
                    return false;
                }
            }
            None => (),
        }
        return true;
    }
}
impl fmt::Display for IfStatement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "if ({}) {}", self.condition, self.body) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.else_body.clone() {
            Some(eb) => match write!(fmt, "\nelse {}", eb) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => (),
        }
        return Ok(());
    }
}
