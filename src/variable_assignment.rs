use super::{expression::Expression, identifier::Identifier};

#[derive(Clone, Default)]
pub struct VariableAssignment {
    pub name: Identifier,
    pub operator: String,
    pub value: Expression
}
impl VariableAssignment {
    pub fn new(name: Identifier, operator: String, value: Expression) -> VariableAssignment {
        return VariableAssignment {
            name,
            operator,
            value
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        if !["+=", "-=", "*=", "/=", "="].contains(&self.operator.as_str()) {
            return false;
        }

        return self.value.is_valid();
    }
}