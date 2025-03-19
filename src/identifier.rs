use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::expression::Expression;

#[derive(Debug, Clone, Default, EnumAsInner)]
pub enum IdentifierAccess {
    Access(Access),
    DerefAccess(Access),
    FunctionCall(FunctionCall),
    #[default]
    Null,
}
impl IdentifierAccess {
    pub fn is_valid(&self) -> bool {
        if self.is_access() {
            return self.as_access().unwrap().is_valid();
        } else if self.is_deref_access() {
            return self.as_deref_access().unwrap().is_valid();
        } else if self.is_function_call() {
            return self.as_function_call().unwrap().is_valid();
        } else {
            return false;
        }
    }
}
impl fmt::Display for IdentifierAccess {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_access() {
            match write!(fmt, "{}", self.as_access().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_deref_access() {
            match write!(fmt, "{}", self.as_deref_access().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_function_call() {
            match write!(fmt, "{}", self.as_function_call().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub struct Access {
    pub name: String,
    pub child: Option<Box<IdentifierAccess>>,
}
impl Access {
    pub fn new(name: String) -> Access {
        return Access {
            name: name,
            child: None,
        };
    }

    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }
        if self.child.is_some() {
            return self.child.clone().unwrap().is_valid();
        }
        return true;
    }
}
impl fmt::Display for Access {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.child.clone() {
            Some(c) => {
                if c.is_access() {
                    match write!(fmt, ".{}", c.as_access().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                } else if c.is_deref_access() {
                    match write!(fmt, "->{}", c.as_deref_access().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                } else if c.is_function_call() {
                    match write!(fmt, ".{}", c.as_function_call().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
            }
            None => (),
        }

        return Ok(());
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
    pub child: Option<Box<IdentifierAccess>>,
}
impl FunctionCall {
    pub fn new(name: String, arguments: Vec<Expression>) -> FunctionCall {
        return FunctionCall {
            name: name,
            arguments: arguments,
            child: None,
        };
    }

    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }
        for i in 0..self.arguments.len() {
            if !self.arguments[i].clone().is_valid() {
                return false;
            }
        }
        match self.child.clone() {
            Some(c) => {
                return c.is_valid();
            }
            None => return true,
        }
    }
}
impl fmt::Display for FunctionCall {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}(", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if self.arguments.len() >= 1 {
            match write!(fmt, "{}", self.arguments[0]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        for i in 1..self.arguments.len() {
            match write!(fmt, ", {}", self.arguments[i]) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, ")") {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.child.clone() {
            Some(c) => {
                if c.is_access() {
                    match write!(fmt, ".{}", c.as_access().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                } else if c.is_deref_access() {
                    match write!(fmt, "->{}", c.as_deref_access().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                } else if c.is_function_call() {
                    match write!(fmt, ".{}", c.as_function_call().unwrap()) {
                        Ok(_) => (),
                        Err(e) => return Err(e),
                    }
                }
            }
            None => (),
        }
        return Ok(());
    }
}
