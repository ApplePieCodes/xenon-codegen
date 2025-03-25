use super::expression::Expression;

#[derive(Clone, Default)]
pub struct ReturnStatement {
    pub value: Expression
}
impl ReturnStatement {
    pub fn new(value: Expression) -> ReturnStatement {
        return ReturnStatement {
            value
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.value.is_valid();
    }
}