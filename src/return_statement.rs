use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct ReturnStatement {
    pub value: Option<Expression>,
}
