use crate::{Visibility, expression::Expression, r#type::Type};

#[derive(Debug, Clone)]
pub struct VariableDefinition {
    pub visibility: Visibility,
    pub name: String,
    pub ty: Type,
    pub value: Option<Expression>,
}
