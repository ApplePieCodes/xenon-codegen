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
            self.as_access().unwrap().is_valid()
        } else if self.is_deref_access() {
            return self.as_deref_access().unwrap().is_valid();
        } else if self.is_function_call() {
            return self.as_function_call().unwrap().is_valid();
        } else {
            return false;
        }
    }

    pub fn set_generic_child(&mut self, child: IdentifierAccess) {
        if self.is_access() {
            let mut temp = self.clone().into_access().unwrap();
            temp.child = Some(Box::new(child));
            *self = IdentifierAccess::Access(temp);
        } else if self.is_deref_access() {
            let mut temp = self.clone().into_deref_access().unwrap();
            temp.child = Some(Box::new(child));
            *self = IdentifierAccess::DerefAccess(temp);
        } else if self.is_function_call() {
            let mut temp = self.clone().into_function_call().unwrap();
            temp.child = Some(Box::new(child));
            *self = IdentifierAccess::FunctionCall(temp);
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
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Access {
    pub name: String,
    pub child: Option<Box<IdentifierAccess>>,
}
impl Access {
    pub fn new(name: String) -> Access {
        Access { name, child: None }
    }

    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }
        if self.child.is_some() {
            return self.child.clone().unwrap().is_valid();
        }
        true
    }
}
impl fmt::Display for Access {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if let Some(c) = self.child.clone() {
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

        Ok(())
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
        FunctionCall {
            name,
            arguments,
            child: None,
        }
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
            Some(c) => c.is_valid(),
            None => true,
        }
    }
}
impl fmt::Display for FunctionCall {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}(", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        if !self.arguments.is_empty() {
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
        if let Some(c) = self.child.clone() {
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
        Ok(())
    }
}
