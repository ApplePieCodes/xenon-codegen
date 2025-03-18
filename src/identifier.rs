use crate::expression::Expression;

#[derive(Debug, Clone)]
pub enum IdentifierAccess {
    Access(Access),
    DerefAccess(Access),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone)]
pub struct Access {
    pub name: String,
    pub child: Option<Box<IdentifierAccess>>,
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
    pub child: Option<Box<IdentifierAccess>>,
}
