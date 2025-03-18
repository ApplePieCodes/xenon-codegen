use crate::{
    identifier::IdentifierAccess, if_statement::IfStatement, loop_statement::LoopStatement,
    return_statement::ReturnStatement, scope::Scope, variable_assignment::VariableAssignment,
    variable_definition::VariableDefinition, while_statement::WhileStatement,
};

#[derive(Debug, Clone, Default)]
pub enum Statement {
    Scope(Scope),
    VariableDefinition(VariableDefinition),
    VariableAssignment(VariableAssignment),
    FunctionCall(IdentifierAccess),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    LoopStatement(LoopStatement),
    ReturnStatement(ReturnStatement),
    BreakStatement,
    ContinueStatement,
    #[default]
    Null,
}
