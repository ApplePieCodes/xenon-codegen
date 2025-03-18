use crate::statement::Statement;

/// Defines a scope
#[derive(Debug, Clone)]
pub struct Scope {
    pub items: Vec<Statement>,
}
