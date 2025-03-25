use enum_as_inner::EnumAsInner;
pub mod attribute;
pub mod block;
pub mod enum_declaration;
pub mod expression;
pub mod function_declaration;
pub mod identifier;
pub mod if_statement;
pub mod impl_declaration;
pub mod loop_statement;
pub mod module;
pub mod program;
pub mod return_statement;
pub mod statement;
pub mod struct_declaration;
pub mod trait_declaration;
pub mod r#type;
pub mod unsafe_block;
pub mod use_statement;
pub mod variable_assignment;
pub mod variable_definition;
pub mod while_statement;

/// Defines the visibility of its owner
#[derive(Debug, Clone, EnumAsInner, Default)]
pub enum Visibility {
    Public,
    Private,
    #[default]
    None
}