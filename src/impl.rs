use crate::{attribute::Attribute, function::Function, r#type::Type};

#[derive(Debug, Clone)]
pub struct Impl {
    pub attrs: Vec<Attribute>,
    pub target: Type,
    /// The trait the impl is implimenting
    pub r#trait: Type,
    pub methods: Vec<Function>,
}
