use super::{identifier::Identifier, statement::Statement, r#type::Type, Visibility};

#[derive(Clone, Default)]
pub struct FunctionDeclaration {
    pub visibility: Visibility,
    pub r#async: bool,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub r#type: Type,
    pub body: Statement
}
impl FunctionDeclaration {
    pub fn new(name: Identifier, r#type: Type, body: Statement) -> FunctionDeclaration {
        return FunctionDeclaration {
            visibility: Visibility::None,
            r#async: false,
            name,
            parameters: vec![],
            r#type,
            body
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        for i in 0..self.parameters.len() {
            if !self.parameters[i].is_valid() {
                return false;
            }
        }

        if !self.r#type.is_valid() {
            return false;
        }

        if !self.body.is_valid() {
            return false;
        }

        return true;
    }
}

#[derive(Clone, Default)]
pub struct Parameter {
    pub name: String,
    pub r#type: Type
}
impl Parameter {
    pub fn new(name: String, r#type: Type) -> Parameter {
        return Parameter {
            name,
            r#type
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.name.is_empty() && self.r#type.is_valid();
    }
}