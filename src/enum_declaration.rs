use super::{attribute::Attribute, identifier::Identifier, r#type::Type, Visibility};

/// An enum type declaration
#[derive(Clone, Default)]
pub struct EnumDeclaration {
    /// A list of Attributes
    pub attributes: Vec<Attribute>,
    /// The visibility of the enum
    pub visibility: Visibility,
    /// The name of the enum
    pub name: Type,
    /// Variants of the enum
    pub variants: Vec<Variant>
}
impl EnumDeclaration {
    /// Creates a new instance of EnumDeclaration
    pub fn new(name: Type) -> EnumDeclaration {
        return EnumDeclaration {
            attributes: vec![],
            visibility: Visibility::None,
            name,
            variants: vec![]
        };
    }

    /// Checks if the instance of EnumDeclaration is valid
    pub fn is_valid(&self) -> bool {
        for i in 0..self.attributes.len() {
            if !self.attributes[i].is_valid() {
                return false;
            }
        }
        if !self.name.is_valid() {
            return false;
        }
        for i in 0..self.variants.len() {
            if !self.variants[i].is_valid() {
                return false;
            }
        }
        return true;
    }
}

/// An enum Variant
#[derive(Clone, Default)]
pub struct Variant {
    /// The name of the variant
    pub name: Identifier,
    /// The contained type, if any
    pub r#type: Option<Type>
}
impl Variant {
    /// Creates a new valid instance of Variant
    pub fn new(name: Identifier) -> Variant {
        return Variant {
            name,
            r#type: None
        };
    }

    /// Checks if the instance of Variant is valid
    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        match self.r#type.as_ref() {
            Some(t) => {
                if !t.is_valid() {
                    return false;
                }
            }
            None => {}
        }
        
        return true;
    }
}