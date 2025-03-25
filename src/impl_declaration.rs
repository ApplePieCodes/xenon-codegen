use super::{attribute::Attribute, function_declaration::FunctionDeclaration, r#type::Type};

#[derive(Clone, Default)]
pub struct ImplDeclaration {
    pub attributes: Vec<Attribute>,
    /// The trait being implimented
    pub r#trait: Option<Type>,
    /// The type to impliment for
    pub r#type: Type,
    pub methods: Vec<FunctionDeclaration>
}
impl ImplDeclaration {
    pub fn new(r#type: Type) -> ImplDeclaration {
        return ImplDeclaration {
            attributes: vec![],
            r#trait: None,
            r#type,
            methods: vec![]
        };
    }

    pub fn is_valid(&self) -> bool {
        for i in 0..self.attributes.len() {
            if !self.attributes[i].is_valid() {
                return false;
            }
        }

        match self.r#trait.as_ref() {
            Some(t) => {
                if !t.is_valid() {
                    return false;
                }
            }
            None => ()
        }

        if !self.r#type.is_valid() {
            return false;
        }

        for i in 0..self.methods.len() {
            if !self.methods[i].is_valid() {
                return false;
            }
        }

        return true;
    }
}