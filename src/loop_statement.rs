use crate::statement::Statement;

#[derive(Debug, Clone)]
pub struct LoopStatement {
    pub body: Box<Statement>,
}
