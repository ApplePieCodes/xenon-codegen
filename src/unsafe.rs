use crate::statement::Statement;

#[derive(Debug, Clone)]
pub struct Unsafe {
    pub body: Statement,
}
