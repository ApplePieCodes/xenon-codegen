use crate::expression::Expression;

#[derive(Debug, Clone)]
pub struct VariableAssignment {
    pub name: String,
    pub value: Expression,
}
