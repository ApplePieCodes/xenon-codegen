use core::fmt;

use crate::statement::Statement;

#[derive(Debug, Clone, Default)]
pub struct Unsafe {
    pub body: Box<Statement>,
}
impl Unsafe {
    pub fn new(body: Statement) -> Unsafe {
        return Unsafe {
            body: Box::new(body),
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.body.is_valid();
    }
}
impl fmt::Display for Unsafe {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "unsafe {}", self.body) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        return Ok(());
    }
}
