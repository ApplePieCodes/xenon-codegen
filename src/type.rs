use core::fmt;

use crate::identifier::IdentifierAccess;

#[derive(Debug, Clone, Default)]
pub struct Type {
    pub name: IdentifierAccess,
    pub typetype: TypeType,
    pub generic_child: Option<Box<Type>>,
}
impl Type {
    pub fn new(name: IdentifierAccess, typetype: TypeType) -> Type {
        return Type {
            name: name,
            typetype: typetype,
            generic_child: None,
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }
        match self.generic_child.clone() {
            Some(g) => {
                if !g.is_valid() {
                    return false;
                }
            }
            None => (),
        }
        return true;
    }
}
impl fmt::Display for Type {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.typetype == TypeType::Pointer {
            match write!(fmt, "*") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        match write!(fmt, "{}", self.name) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        match self.generic_child.clone() {
            Some(g) => match write!(fmt, "<{}>", g) {
                Ok(_) => (),
                Err(e) => return Err(e),
            },
            None => (),
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum TypeType {
    #[default]
    Normal,
    Pointer,
}
