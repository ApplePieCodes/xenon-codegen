use super::{attribute::Attribute, function_declaration::FunctionDeclaration, r#type::Type, variable_definition::VariableDefinition, Visibility};

#[derive(Clone, Default)]
pub struct TraitDeclaration {
    pub attributes: Vec<Attribute>,
    pub visibility: Visibility,
    pub name: Type,
    pub fields: Vec<VariableDefinition>,
    pub methods: Vec<FunctionDeclaration>
}
impl TraitDeclaration {
    pub fn new(name: Type) -> TraitDeclaration {
        return TraitDeclaration {
            attributes: vec![],
            visibility: Visibility::None,
            name,
            fields: vec![],
            methods: vec![]
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

        for i in 0..self.fields.len() {
            if !self.fields[i].is_valid() {
                return false;
            }
        }
        
        for i in 0..self.methods.len() {
            if !self.methods[i].is_valid() {
                return false;
            }
        }

        return true;
    }
}