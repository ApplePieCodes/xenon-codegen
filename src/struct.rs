use crate::{
    Visibility, attribute::Attribute, r#type::Type, variable_definition::VariableDefinition,
};

#[derive(Debug, Clone)]
pub struct Struct {
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Type,
    pub base: Type,
    pub properties: Vec<VariableDefinition>,
}
