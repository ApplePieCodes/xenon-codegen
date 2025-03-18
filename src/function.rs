use crate::{
    Visibility, attribute::Attribute, statement::Statement, r#type::Type,
    variable_definition::VariableDefinition,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub r#async: bool,
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub arguments: Vec<VariableDefinition>,
    pub returns: Option<Type>,
    pub body: Option<Statement>,
}
