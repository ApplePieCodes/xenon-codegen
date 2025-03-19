use core::fmt;

use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}
impl ReturnStatement {
    pub fn new() -> ReturnStatement {
        return ReturnStatement { value: None };
    }

    pub fn is_valid(&self) -> bool {
        match self.value.clone() {
            Some(v) => {
                return v.clone().is_valid();
            }
            None => return true,
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
        return Ok(());
    }
}
