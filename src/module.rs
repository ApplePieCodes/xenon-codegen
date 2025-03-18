use crate::{
    Visibility, attribute::Attribute, r#enum::Enum, function::Function, r#impl::Impl,
    r#struct::Struct, r#trait::Trait,
};

/// A module item
#[derive(Debug, Clone)]
pub enum ModuleItem {
    Module(Module),
    Struct(Struct),
    Function(Function),
    Trait(Trait),
    Enum(Enum),
    Impl(Impl),
}

/// Defines a module
#[derive(Debug, Clone)]
pub struct Module {
    pub attrs: Vec<Attribute>,
    /// Module Visibility
    pub visibility: Visibility,
    /// Module Name
    pub name: String,
    pub items: Vec<ModuleItem>,

    pub attributes: Vec<Attribute>,
}
