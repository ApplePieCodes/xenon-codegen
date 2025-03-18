use crate::{expression::Expression, statement::Statement};

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Box<Statement>,
    pub else_body: Option<Box<Statement>>,
}
