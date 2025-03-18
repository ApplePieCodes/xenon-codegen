use crate::{
    Visibility, attribute::Attribute, function::Function, r#type::Type,
    variable_definition::VariableDefinition,
};

#[derive(Debug, Clone)]
pub struct Trait {
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Type,
    pub fields: Vec<VariableDefinition>,
    pub methods: Vec<Function>,
}
