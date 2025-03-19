use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::{
    identifier::IdentifierAccess, if_statement::IfStatement, loop_statement::LoopStatement,
    return_statement::ReturnStatement, scope::Scope, r#unsafe::Unsafe,
    variable_assignment::VariableAssignment, variable_definition::VariableDefinition,
    while_statement::WhileStatement,
};

#[derive(Debug, Clone, Default, EnumAsInner)]
pub enum Statement {
    Scope(Scope),
    VariableDefinition(VariableDefinition),
    VariableAssignment(VariableAssignment),
    FunctionCall(IdentifierAccess),
    IfStatement(IfStatement),
    WhileStatement(WhileStatement),
    LoopStatement(LoopStatement),
    ReturnStatement(ReturnStatement),
    Unsafe(Unsafe),
    BreakStatement,
    ContinueStatement,
    #[default]
    Null,
}
impl Statement {
    pub fn is_valid(&self) -> bool {
        if self.is_scope() {
            return self.as_scope().unwrap().is_valid();
        } else if self.is_variable_definition() {
            return self.as_variable_assignment().unwrap().is_valid();
        } else if self.is_variable_assignment() {
            return self.as_variable_assignment().unwrap().is_valid();
        } else if self.is_function_call() {
            return self.as_function_call().unwrap().is_valid();
        } else if self.is_if_statement() {
            return self.as_if_statement().unwrap().is_valid();
        } else if self.is_while_statement() {
            return self.as_while_statement().unwrap().is_valid();
        } else if self.is_loop_statement() {
            return self.as_loop_statement().unwrap().is_valid();
        } else if self.is_return_statement() {
            return self.as_return_statement().unwrap().is_valid();
        } else if self.is_unsafe() {
            return self.as_unsafe().unwrap().is_valid();
        } else if self.is_break_statement() {
            return true;
        } else if self.is_continue_statement() {
            return true;
        } else {
            return false;
        }
    }
}
impl fmt::Display for Statement {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_scope() {
            match write!(fmt, "{}", self.as_scope().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_variable_definition() {
            match write!(fmt, "{};", self.as_variable_definition().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_variable_assignment() {
            match write!(fmt, "{};", self.as_variable_assignment().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_function_call() {
            match write!(fmt, "{};", self.as_function_call().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_if_statement() {
            match write!(fmt, "{}", self.as_if_statement().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_while_statement() {
            match write!(fmt, "{}", self.as_while_statement().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_loop_statement() {
            match write!(fmt, "{}", self.as_loop_statement().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_return_statement() {
            match write!(fmt, "{};", self.as_return_statement().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_unsafe() {
            match write!(fmt, "{}", self.as_unsafe().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_break_statement() {
            match write!(fmt, "break;") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_continue_statement() {
            match write!(fmt, "continue;") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        return Ok(());
    }
}
