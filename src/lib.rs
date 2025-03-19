use core::fmt;
use enum_as_inner::EnumAsInner;

pub mod attribute;
pub mod case;
pub mod r#enum;
pub mod expression;
pub mod function;
pub mod identifier;
pub mod if_statement;
pub mod r#impl;
pub mod loop_statement;
pub mod module;
pub mod return_statement;
pub mod scope;
pub mod statement;
pub mod r#struct;
pub mod switch_statement;
pub mod r#trait;
pub mod r#type;
pub mod r#unsafe;
pub mod variable_assignment;
pub mod variable_definition;
pub mod while_statement;

/// Defines the visibility of its owner
#[derive(Debug, Clone, EnumAsInner, Default)]
pub enum Visibility {
    Public,
    #[default]
    Private,
}
impl fmt::Display for Visibility {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_private() {
            match write!(fmt, "private") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_public() {
            match write!(fmt, "public") {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use core::panic;
    use std::{thread::Scope, vec};

    use galvanic_test::test_suite;

    use crate::{
        Visibility, attribute::Attribute, expression::UnaryOperation, identifier::IdentifierAccess,
        loop_statement::LoopStatement, module::Module, statement, r#type::Type,
    };

    test_suite! {
        name attribute;
        use crate::attribute::Attribute;

        test attribute_test() {
            let attribute = Attribute::new("attribute");
            assert!(attribute.to_string() == "#[attribute]\n");
        }

        test attribute_test_with_value() {
            let mut attribute = Attribute::new("attribute");
            attribute.value = Some(String::from("value"));
            assert!(attribute.to_string() == "#[attribute(value)]\n");
        }
    }

    test_suite! {
        name case;
        use crate::{expression::{Expression, IntegerLiteral}, case::Case, scope::Scope};

        test case_test() {
            let case = Case::new(
                Expression::IntegerLiteral(IntegerLiteral::new(0)),
                Scope::new(),
            );
            assert!(case.to_string() == "case 0 {\n}");
        }
    }

    test_suite! {
        name r#enum;
        use crate::{r#enum::Enum, Visibility};

        test private_enum_test() {
            let enu = Enum::new("test".to_string());
            assert!(enu.to_string() == "private enum test {}");
        }

        test public_enum_test() {
            let mut enu = Enum::new("test".to_string());
            enu.visibility = Visibility::Public;
            assert!(enu.to_string() == "public enum test {}");
        }
    }

    test_suite! {
        name expressions;
        use crate::{expression::{Expression, IntegerLiteral, FloatLiteral, StringLiteral, BooleanLiteral, Parentheses, UnaryOperation, BinaryOperation}};

        test integer_literal_test() {
            let int = IntegerLiteral::new(10);
            assert!(int.to_string() == "10");
        }

        test float_literal_test() {
            let float = FloatLiteral::new(1.0);
            assert!(float.to_string() == "1");
        }

        test string_literal_test() {
            let string = StringLiteral::new("Hello World!".to_string());
            assert!(string.to_string() == "\"Hello World!\"");
        }

        test boolean_literal_test() {
            let bol = BooleanLiteral::new(true);
            assert!(bol.to_string() == "true");
        }

        test parentheses_test() {
            let par = Parentheses::new(Expression::IntegerLiteral(IntegerLiteral::new(12)));
            assert!(par.to_string() == "(12)");
        }

        test unary_operation_test() {
            let una = UnaryOperation::new("-".to_string(), Expression::IntegerLiteral(IntegerLiteral::new(42)));
            assert!(una.to_string() == "-42");
        }

        test binary_operation_test() {
            let bin = BinaryOperation::new(Expression::IntegerLiteral(IntegerLiteral::new(2)), "+".to_string(), Expression::IntegerLiteral(IntegerLiteral::new(2)));
            assert!(bin.to_string() == "2 + 2");
        }
    }

    test_suite! {
        name function;
        use crate::{Visibility, function::Function, r#type::{Type, TypeType}, identifier::{Access, IdentifierAccess}, statement::Statement, scope::Scope};

        test public_function() {
            let mut func = Function::new("test".to_string(), Type::new(IdentifierAccess::Access(Access::new("void".to_string())), TypeType::Normal), Statement::Scope(Scope::new()));
            func.visibility = Visibility::Public;
            assert!(func.to_string() == "public fn test() -> void {\n}");
        }

        test private_function() {
            let func = Function::new("test".to_string(), Type::new(IdentifierAccess::Access(Access::new("void".to_string())), TypeType::Normal), Statement::Scope(Scope::new()));
            assert!(func.to_string() == "private fn test() -> void {\n}");
        }
    }

    test_suite! {
        name identifier;
        use crate::{identifier::{IdentifierAccess, Access, FunctionCall}, expression::{Expression, BinaryOperation, IntegerLiteral}};

        test access() {
            let ident = Access::new("identifier".to_string());
            assert!(ident.to_string() == "identifier");
        }

        test function_call() {
            let mut ident = FunctionCall::new("func".to_string(), vec![]);
            ident.arguments.push(Expression::BinaryOperation(BinaryOperation::new(Expression::IntegerLiteral(IntegerLiteral::new(2)), "+".to_string(), Expression::IntegerLiteral(IntegerLiteral::new(2)))));
            assert!(ident.to_string() == "func(2 + 2)");
        }

        test access_and_deref() {
            let mut ident = Access::new("one".to_string());
            ident.child = Some(Box::new(IdentifierAccess::DerefAccess(Access::new("two".to_string()))));
            assert!(ident.to_string() == "one->two")
        }
    }

    test_suite! {
        name if_statement;
        use crate::{if_statement::IfStatement, expression::{Expression, BooleanLiteral}, scope::Scope, statement::Statement};

        test if_statement() {
            let statement = IfStatement::new(Expression::BooleanLiteral(BooleanLiteral::new(true)), Statement::Scope(Scope::new()));
            assert!(statement.to_string() == "if (true) {\n}")
        }

        test else_statement() {
            let mut statement = IfStatement::new(Expression::BooleanLiteral(BooleanLiteral::new(true)), Statement::Scope(Scope::new()));
            statement.else_body = Some(Box::new(Statement::Scope(Scope::new())));
            assert!(statement.to_string() == "if (true) {\n}\nelse {\n}")
        }
    }

    test_suite! {
        name r#impl;
        use crate::{r#impl::Impl, r#type::{Type, TypeType}, identifier::{IdentifierAccess, Access}, r#trait::Trait};

        test r#impl() {
            let imp = Impl::new(Type::new(IdentifierAccess::Access(Access::new("Test".to_string())), TypeType::Normal));
            assert!(imp.to_string() == "impliment Test {\n}");
        }

        test impl_for() {
            let mut imp = Impl::new(Type::new(IdentifierAccess::Access(Access::new("Test".to_string())), TypeType::Normal));
            imp.r#trait = Some(Type::new(IdentifierAccess::Access(Access::new("Treat".to_string())), TypeType::Normal));
            assert!(imp.to_string() == "impliment Treat for Test {\n}");
        }
    }

    test_suite! {
        name loop_statement;
        use crate::{loop_statement::LoopStatement, statement::Statement, scope::Scope};

        test loop_statement() {
            let lop = LoopStatement::new(Statement::Scope(Scope::new()));
            assert!(lop.to_string() == "loop {\n}")
        }
    }

    test_suite! {
        name module;
        use crate::{module::{Module, ModuleItem}, Visibility};

        test public_module() {
            let mut modu = Module::new("test".to_string());
            modu.visibility = Visibility::Public;
            assert!(modu.to_string() == "public module test {}");
        }

        test private_module() {
            let modu = Module::new("test".to_string());
            assert!(modu.to_string() == "private module test {}");
        }
    }

    test_suite! {
        name return_statement;
        use crate::{return_statement::ReturnStatement, expression::{Expression, IntegerLiteral}};

        test return_no_value() {
            let ret = ReturnStatement::new();
            assert!(ret.to_string() == "return;");
        }

        test return_value() {
            let mut ret = ReturnStatement::new();
            ret.value = Some(Expression::IntegerLiteral(IntegerLiteral::new(0)));
            assert!(ret.to_string() == "return 0;");
        }
    }

    test_suite! {
        name scope;
        use crate::scope::Scope;

        test scope_test() {
            let scope = Scope::new();
            assert!(scope.to_string() == "{\n}");
        }
    }

    test_suite! {
        name r#struct;
        use crate::{Visibility, r#struct::Struct, r#type::{Type, TypeType}, identifier::{IdentifierAccess, Access}};

        test public_struct() {
            let mut struc = Struct::new(Type::new(IdentifierAccess::Access(Access::new("Test".to_string())), TypeType::Normal));
            struc.visibility = Visibility::Public;
            assert!(struc.to_string() == "public struct Test {\n}");
        }

        test private_struct() {
            let struc = Struct::new(Type::new(IdentifierAccess::Access(Access::new("Test".to_string())), TypeType::Normal));
            assert!(struc.to_string() == "private struct Test {\n}");
        }
    }

    test_suite! {
        name switch_statement;
        use crate::{switch_statement::SwitchStatement, expression::{Expression, IntegerLiteral}};

        test switch_statement() {
            let statement = SwitchStatement::new(Expression::IntegerLiteral(IntegerLiteral::new(0)));
            assert!(statement.to_string() == "switch (0) {\n}")
        }
    }

    test_suite! {
        name r#trait;
        use crate::{r#trait::Trait, r#type::{Type, TypeType}, identifier::{IdentifierAccess, Access}, Visibility};

        test public_trait() {
            let mut trai = Trait::new(Type::new(IdentifierAccess::Access(Access::new("Trait".to_string())), TypeType::Normal));
            trai.visibility = Visibility::Public;
            assert!(trai.to_string() == "public trait Trait {\n}");
        }

        test private_trait() {
            let trai = Trait::new(Type::new(IdentifierAccess::Access(Access::new("Trait".to_string())), TypeType::Normal));
            assert!(trai.to_string() == "private trait Trait {\n}");
        }
    }

    test_suite! {
        name r#type;
        use crate::{r#type::{Type, TypeType}, identifier::{IdentifierAccess, Access}};

        test r#type() {
            let typ = Type::new(IdentifierAccess::Access(Access::new("Type".to_string())), TypeType::Normal);
            assert!(typ.to_string() == "Type")
        }

        test generic_type() {
            let mut typ = Type::new(IdentifierAccess::Access(Access::new("Type".to_string())), TypeType::Normal);
            typ.generic_child = Some(Box::new(Type::new(IdentifierAccess::Access(Access::new("Type".to_string())), TypeType::Normal)));
            assert!(typ.to_string() == "Type<Type>")
        }
    }

    test_suite! {
        name r#unsafe;
        use crate::{r#unsafe::Unsafe, scope::Scope, statement::Statement};

        test r#unsafe() {
            let unsaf = Unsafe::new(Statement::Scope(Scope::new()));
            assert!(unsaf.to_string() == "unsafe {\n}");
        }
    }

    test_suite! {
        name variable_assignment;
        use crate::{variable_assignment::VariableAssignment, identifier::{IdentifierAccess, Access}, expression::{Expression, IntegerLiteral}};

        test variable_assignment() {
            let var_ass = VariableAssignment::new(IdentifierAccess::Access(Access::new("x".to_string())), "=".to_string(), Expression::IntegerLiteral(IntegerLiteral::new(0)));
            assert!(var_ass.to_string() == "x = 0")
        }
    }

    test_suite! {
        name variable_definition;
        use crate::{variable_definition::VariableDefinition, r#type::{Type, TypeType}, identifier::{IdentifierAccess, Access}};

        test variable_definition() {
            let mut var_def = VariableDefinition::new("variable".to_string());
            var_def.ty = Some(Type::new(IdentifierAccess::Access(Access::new("i32".to_string())), TypeType::Normal));
            assert!(var_def.to_string() == "let variable: i32");
        }
    }

    test_suite! {
        name while_statement;
        use crate::{statement::Statement, while_statement::WhileStatement, scope::Scope, expression::{Expression, BooleanLiteral}};

        test while_statement() {
            let statement = WhileStatement::new(Expression::BooleanLiteral(BooleanLiteral::new(true)), Statement::Scope(Scope::new()));
            assert!(statement.to_string() == "while (true) {\n}");
        }
    }
}
