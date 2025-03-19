use core::fmt;

use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}
impl Default for ReturnStatement {
    fn default() -> Self {
        Self::new()
    }
}

impl ReturnStatement {
    pub fn new() -> ReturnStatement {
        ReturnStatement { value: None }
    }

    pub fn is_valid(&self) -> bool {
        match self.value.clone() {
            Some(v) => v.clone().is_valid(),
            None => true,
        }
    }
}
impl fmt::Display for ReturnStatement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "return") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.value.clone() {
            Some(v) => match write!(fmt, " {};", v) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => match write!(fmt, ";") {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
        }
        Ok(())
    }
}
