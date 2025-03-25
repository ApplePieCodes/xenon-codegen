use super::{expression::Expression, statement::Statement};

#[derive(Clone, Default)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>
}
impl IfStatement {
    pub fn new(condition: Expression, body: Statement) -> IfStatement {
        return IfStatement {
            condition,
            body: Box::new(body),
            else_body: None
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.condition.is_valid() {
            return false;
        }

        if !self.body.is_valid() {
            return false;
        }

        match self.else_body.as_ref() {
            Some(e) => {
                return e.is_valid();
            }
            None => {
                return true;
            }
        }
    }
}