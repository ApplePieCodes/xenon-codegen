use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::{
    Visibility, attribute::Attribute, r#enum::Enum, function::Function, r#impl::Impl,
    r#struct::Struct, r#trait::Trait, variable_definition::VariableDefinition,
};

/// A module item
#[derive(Debug, Clone, Default, EnumAsInner)]
pub enum ModuleItem {
    Module(Module),
    Struct(Struct),
    Function(Function),
    VariableDefinition(VariableDefinition),
    Trait(Trait),
    Enum(Enum),
    Impl(Impl),
    #[default]
    Null,
}
impl ModuleItem {
    pub fn is_valid(&self) -> bool {
        if self.is_module() {
            self.as_module().unwrap().is_valid()
        } else if self.is_struct() {
            return self.as_struct().unwrap().is_valid();
        } else if self.is_function() {
            return self.as_function().unwrap().is_valid();
        } else if self.is_variable_definition() {
            return self.as_variable_definition().unwrap().is_valid();
        } else if self.is_trait() {
            return self.as_trait().unwrap().is_valid();
        } else if self.is_enum() {
            return self.as_enum().unwrap().is_valid();
        } else if self.is_impl() {
            return self.as_impl().unwrap().is_valid();
        } else {
            return false;
        }
    }
}
impl fmt::Display for ModuleItem {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_module() {
            match write!(fmt, "{}", self.as_module().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_struct() {
            match write!(fmt, "{}", self.as_struct().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_function() {
            match write!(fmt, "{}", self.as_function().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_variable_definition() {
            match write!(
                fmt,
                "{} {};",
                self.as_variable_definition().unwrap().visibility,
                self.as_variable_definition().unwrap()
            ) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_trait() {
            match write!(fmt, "{}", self.as_trait().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_enum() {
            match write!(fmt, "{}", self.as_enum().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_impl() {
            match write!(fmt, "{}", self.as_impl().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
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
}
impl Module {
    pub fn new(name: String) -> Module {
        Module {
            attrs: vec![],
            visibility: Visibility::Private,
            name,
            items: vec![],
        }
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        if self.name.is_empty() {
            return false;
        }
        for i in 0..self.items.len() {
            if !self.items[i].is_valid() {
                return false;
            }
        }
        true
    }
}
impl fmt::Display for Module {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.attrs.len() {
            match write!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "{} module {} {{", self.visibility, self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        for i in 0..self.items.len() {
            match write!(fmt, "{}", self.items[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "}}") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
