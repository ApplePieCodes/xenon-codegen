use core::fmt;

use crate::{
    Visibility, attribute::Attribute, statement::Statement, r#type::Type,
    variable_definition::VariableDefinition,
};

#[derive(Debug, Clone, Default)]
pub struct Function {
    pub r#async: bool,
    pub attrs: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: String,
    pub arguments: Vec<Attribute>,
    pub returns: Type,
    pub body: Statement,
}
impl Function {
    pub fn new(name: String, arguments: Vec<VariableDefinition>, returns: Type, body: Statement) -> Function {
        return Function {
            r#async: false,
            attrs: vec![],
            visibility: Visibility::Private,
            name: name,
            arguments: arguments,
            returns: returns,
            body: body
        };
    }

    pub fn is_valid(&mut self) -> bool {
        for i in 0..self.attrs.len() {
            if !self.attrs[i].is_valid() {
                return false;
            }
        }
        if self.name.is_empty() {
            return false;
        }
        for i in 0..self.arguments.len() {
            if !self.arguments[i].is_valid() {
                return false;
            }
        }
        if !self.returns.is_valid() {
            return false;
        }
        if !self.body.is_valid() {
            return false;
        }

        return true;
    }
}

impl fmt::Display for Function {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.attrs.len() {
            match writeln!(fmt, "{}", self.attrs[i]) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }
        if self.r#async {
            match write!(fmt, "async") {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
        }
        match write!(fmt, "{} fn {}(", self.visibility, self.name) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        if self.arguments.len() >= 1 {
            match write!(fmt, "{}", self.arguments[0]) {
                Ok(_) => (),
                Err(e) => return Err(e)
            }
            for i in 1..self.arguments.len() {
                match write!(fmt, ", {}", self.arguments[i]) {
                    Ok(_) => (),
                    Err(e) => return Err(e)
                }
            }
        }
        match write!(fmt, ") -> {}", self.returns) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        match write!(fmt, "{}", self.body) {
            Ok(_) => (),
            Err(e) => return Err(e)
        }
        
        return Ok(());
    }
}

#[derive(Debug, Clone, Default)]
pub struct Argument {
    pub name: String,
    pub r#type: Type
}