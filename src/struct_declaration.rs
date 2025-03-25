use super::{r#type::Type, variable_definition::VariableDefinition, Visibility};

#[derive(Clone, Default)]
pub struct StructDeclaration {
    pub visibility: Visibility,
    pub name: Type,
    pub fields: Vec<VariableDefinition>
}
impl StructDeclaration {
    pub fn new(name: Type) -> StructDeclaration {
        return StructDeclaration {
            visibility: Visibility::None,
            name,
            fields: vec![]
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        for i in 0..self.fields.len() {
            if !self.fields[i].is_valid() {
                return false;
            }
        }

        return true;
    }
}