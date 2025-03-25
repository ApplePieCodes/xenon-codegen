use enum_as_inner::EnumAsInner;

use super::{block::Block, identifier::Identifier, if_statement::IfStatement, loop_statement::LoopStatement, return_statement::ReturnStatement, unsafe_block::UnsafeBlock, variable_assignment::VariableAssignment, variable_definition::VariableDefinition, while_statement::WhileStatement};

#[derive(Clone, Default, EnumAsInner)]
pub enum Statement {
   Block(Block),
   VariableDefinition(VariableDefinition),
   VariableAssignment(VariableAssignment),
   FunctionCall(Identifier),
   IfStatement(IfStatement),
   WhileStatement(WhileStatement),
   LoopStatement(LoopStatement),
   ReturnStatement(ReturnStatement),
   UnsafeBlock(UnsafeBlock),
   BreakStatement,
   ContinueStatement,
   #[default]
   Null
}
impl Statement {
   pub fn is_valid(&self) -> bool {
      if self.is_block() {
         return self.as_block().unwrap().is_valid();
      }
      else if self.is_variable_definition() {
         return self.as_variable_definition().unwrap().is_valid();
      }
      else if self.is_variable_assignment() {
         return self.as_variable_assignment().unwrap().is_valid();
      }
      else if self.is_function_call() {
         return self.as_function_call().unwrap().is_valid();
      }
      else if self.is_if_statement() {
         return self.as_if_statement().unwrap().is_valid();
      }
      else if self.is_while_statement() {
         return self.as_while_statement().unwrap().is_valid();
      }
      else if self.is_loop_statement() {
         return self.as_loop_statement().unwrap().is_valid();
      }
      else if self.is_return_statement() {
         return self.as_return_statement().unwrap().is_valid();
      }
      else if self.is_unsafe_block() {
         return self.as_unsafe_block().unwrap().is_valid();
      }
      else if self.is_break_statement() {
         return true;
      }
      else if self.is_continue_statement() {
         return true;
      }
      else {
         return false;
      }
   }
}