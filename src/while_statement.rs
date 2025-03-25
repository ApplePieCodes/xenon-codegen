use super::{expression::Expression, statement::Statement};

#[derive(Clone, Default)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Box<Statement>
}
impl WhileStatement {
    pub fn new(condition: Expression, body: Statement) -> WhileStatement {
        return WhileStatement {
            condition,
            body: Box::new(body)
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.condition.is_valid() && self.body.is_valid();
    }
}