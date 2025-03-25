use enum_as_inner::EnumAsInner;

use crate::identifier::Identifier;

use super::{attribute::Attribute, enum_declaration::EnumDeclaration, function_declaration::FunctionDeclaration, impl_declaration::ImplDeclaration, struct_declaration::StructDeclaration, trait_declaration::TraitDeclaration, use_statement::UseStatement, Visibility};

#[derive(Clone, Default)]
pub struct Module {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Identifier,
    pub items: Vec<ModuleItem>
}
impl Module {
    pub fn new(name: Identifier) -> Module {
        return Module {
            attributes: vec![],
            visibility: Visibility::None,
            name,
            items: vec![]
        };
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attributes.len() {
            if !self.attributes[i].is_valid() {
                return false;
            }
        }

        if !self.name.is_valid() {
            return false;
        }

        for i in 0..self.items.len() {
            if !self.items[i].is_valid() {
                return false;
            }
        }

        return true;
    }
}

#[derive(Clone, Default, EnumAsInner)]
pub enum ModuleItem {
    Module(Module),
    UseStatement(UseStatement),
    FunctionDeclaration(FunctionDeclaration),
    StructDeclaration(StructDeclaration),
    ImplDeclaration(ImplDeclaration),
    EnumDeclaration(EnumDeclaration),
    TraitDeclaration(TraitDeclaration),
    #[default]
    None
}
impl ModuleItem {
    pub fn is_valid(&self) -> bool {
        if self.is_module() {
            return self.as_module().unwrap().is_valid();
        }
        else if self.is_use_statement() {
            return self.as_use_statement().unwrap().is_valid();
        }
        else if self.is_function_declaration() {
            return self.as_function_declaration().unwrap().is_valid();
        }
        else if self.is_struct_declaration() {
            return self.as_struct_declaration().unwrap().is_valid();
        }
        else if self.is_impl_declaration() {
            return self.as_impl_declaration().unwrap().is_valid();
        }
        else if self.is_enum_declaration() {
            return self.as_enum_declaration().unwrap().is_valid();
        }
        else if self.is_trait_declaration() {
            return self.as_trait_declaration().unwrap().is_valid();
        }
        else {
            return false;
        }
    }
}