use super::statement::Statement;

#[derive(Clone, Default)]
pub struct LoopStatement {
    pub body: Box<Statement>
}
impl LoopStatement {
    pub fn new(body: Statement) -> LoopStatement {
        return LoopStatement {
            body: Box::new(body)
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.body.is_valid();
    }
}