use core::fmt;

use crate::statement::Statement;

#[derive(Debug, Clone, Default)]
pub struct LoopStatement {
    pub body: Box<Statement>,
}
impl LoopStatement {
    pub fn new(body: Statement) -> LoopStatement {
        return LoopStatement {
            body: Box::new(body),
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.body.is_valid() {
            return false;
        }
        return true;
    }
}
impl fmt::Display for LoopStatement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "loop {}", self.body) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
