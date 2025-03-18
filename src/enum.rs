use crate::{attribute::Attribute, r#type::Type};

#[derive(Debug, Clone)]
pub struct Enum {
    pub attrs: Vec<Attribute>,
    pub name: Type,
    pub variants: Vec<Variant>,
}

#[derive(Debug, Clone)]
pub struct Variant {
    pub attrs: Vec<Attribute>,
    pub name: String,
}
