use super::{expression::Expression, identifier::Identifier, r#type::Type, Visibility};

#[derive(Clone, Default)]
pub struct VariableDefinition {
    pub visibility: Visibility,
    pub name: Identifier,
    pub r#type: Option<Type>,
    pub value: Option<Expression>
}
impl VariableDefinition {
    pub fn new(name: Identifier) -> VariableDefinition {
        return VariableDefinition {
            visibility: Visibility::None,
            name,
            r#type: None,
            value: None
        };
    }

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
            None => {

            }
        }

        match self.value.as_ref() {
            Some(v) => {
                if !v.is_valid() {
                    return false;
                }
            }
            None => {

            }
        }

        return true;
    }
}