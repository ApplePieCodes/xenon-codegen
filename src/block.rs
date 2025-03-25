use super::statement::Statement;

/// A block of code
#[derive(Clone, Default)]
pub struct Block {
    /// The contained statements
    pub statements: Vec<Statement>
}
impl Block {
    /// Create a new, valid instance of Block
    pub fn new() -> Block {
        return Block {
            statements: vec![]
        };
    }

    /// Check if the instance of Block is valid
    pub fn is_valid(&self) -> bool {
        for i in 0..self.statements.len() {
            if !self.statements[i].is_valid() {
                return false;
            }
        }
        return true;
    }
}