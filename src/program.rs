use enum_as_inner::EnumAsInner;

use super::{module::Module, use_statement::UseStatement};

pub struct Program {
    pub file_name: String,
    pub items: Vec<TopLevelItem>
}
impl Program {
    pub fn new(file_name: &str) -> Program {
        return Program {
            file_name: file_name.to_string(),
            items: vec![]
        };
    }

    pub fn is_valid(&self) -> bool {
        if self.file_name.is_empty() {
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
pub enum TopLevelItem {
    UseStatement(UseStatement),
    Module(Module),
    #[default]
    Null
}
impl TopLevelItem {
    pub fn is_valid(&self) -> bool {
        if self.is_use_statement() {
            return self.as_use_statement().unwrap().is_valid();
        }
        else if self.is_module() {
            return self.as_module().unwrap().is_valid();
        }
        else {
            return false;
        }
    }
}