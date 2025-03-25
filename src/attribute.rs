use crate::identifier::Identifier;

/// An attribute applied to it's owner
#[derive(Clone, Default)]
pub struct Attribute {
    /// The name of the attribute
    pub name: Identifier,
    /// The value of the attribute, if any
    pub value: Option<Identifier>
}
impl Attribute {
    /// Create a new, valid instance of Attribute
    pub fn new(name: Identifier) -> Attribute {
        return Attribute {
            name,
            value: None
        };
    }

    /// Check if the instance of Attribute is valid
    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }
        match self.value.as_ref() {
            Some(i) => {
                return i.is_valid();
            }
            None => {
                return true;
            }
        }
    }
}