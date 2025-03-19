use core::fmt;

use enum_as_inner::EnumAsInner;

use crate::identifier::IdentifierAccess;

#[derive(Debug, Clone, Default, EnumAsInner)]
pub enum Expression {
    IntegerLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(BooleanLiteral),
    Parentheses(Parentheses),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Identifier(IdentifierAccess),
    #[default]
    Null,
}
impl Expression {
    pub fn is_valid(&self) -> bool {
        if self.is_integer_literal() {
            self.as_integer_literal().unwrap().is_valid()
        } else if self.is_float_literal() {
            return self.as_float_literal().unwrap().is_valid();
        } else if self.is_string_literal() {
            return self.as_string_literal().unwrap().is_valid();
        } else if self.is_boolean_literal() {
            return self.as_boolean_literal().unwrap().is_valid();
        } else if self.is_parentheses() {
            return self.as_parentheses().unwrap().is_valid();
        } else if self.is_unary_operation() {
            return self.as_unary_operation().unwrap().is_valid();
        } else if self.is_binary_operation() {
            return self.as_binary_operation().unwrap().is_valid();
        } else if self.is_identifier() {
            return self.as_identifier().unwrap().is_valid();
        } else {
            return false;
        }
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_integer_literal() {
            match write!(fmt, "{}", self.as_integer_literal().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_float_literal() {
            match write!(fmt, "{}", self.as_float_literal().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_string_literal() {
            match write!(fmt, "{}", self.as_string_literal().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_boolean_literal() {
            match write!(fmt, "{}", self.as_boolean_literal().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_unary_operation() {
            match write!(fmt, "{}", self.as_unary_operation().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_binary_operation() {
            match write!(fmt, "{}", self.as_binary_operation().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        } else if self.is_identifier() {
            match write!(fmt, "{}", self.as_identifier().unwrap()) {
                Ok(_) => (),
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct IntegerLiteral {
    pub value: i64,
}
impl IntegerLiteral {
    pub fn new(val: i64) -> IntegerLiteral {
        IntegerLiteral { value: val }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}
impl fmt::Display for IntegerLiteral {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}", self.value) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct FloatLiteral {
    pub value: f64,
}
impl FloatLiteral {
    pub fn new(val: f64) -> FloatLiteral {
        FloatLiteral { value: val }
    }

    pub fn is_valid(&self) -> bool {
        self.value.is_normal() || self.value == 0.0
    }
}
impl fmt::Display for FloatLiteral {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}", self.value) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct StringLiteral {
    pub value: String,
}
impl StringLiteral {
    pub fn new(val: String) -> StringLiteral {
        StringLiteral { value: val }
    }
    pub fn is_valid(&self) -> bool {
        true
    }
}
impl fmt::Display for StringLiteral {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "\"{}\"", self.value) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct BooleanLiteral {
    pub value: bool,
}
impl BooleanLiteral {
    pub fn new(val: bool) -> BooleanLiteral {
        BooleanLiteral { value: val }
    }

    pub fn is_valid(&self) -> bool {
        true
    }
}
impl fmt::Display for BooleanLiteral {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}", self.value) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct UnaryOperation {
    pub operator: String,
    pub term: Box<Expression>,
}
impl UnaryOperation {
    pub fn new(op: String, val: Expression) -> UnaryOperation {
        UnaryOperation {
            operator: op,
            term: Box::new(val),
        }
    }

    pub fn is_valid(&self) -> bool {
        if !["+", "-", "*", "&"].contains(&self.operator.as_str()) {
            return false;
        }
        self.term.is_valid()
    }
}
impl fmt::Display for UnaryOperation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{}{}", self.operator, self.term) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Parentheses {
    pub value: Box<Expression>,
}
impl Parentheses {
    pub fn new(expr: Expression) -> Parentheses {
        Parentheses {
            value: Box::new(expr),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.value.is_valid()
    }
}
impl fmt::Display for Parentheses {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "({})", self.value) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Default)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>,
}
impl BinaryOperation {
    pub fn new(left: Expression, op: String, right: Expression) -> BinaryOperation {
        BinaryOperation {
            left: Box::new(left),
            operator: op,
            right: Box::new(right),
        }
    }

    pub fn is_valid(&self) -> bool {
        self.left.is_valid()
            && self.right.is_valid()
            && ["+", "-", "*", "/", "==", "<", "<=", ">", ">=", "&&", "||"]
                .contains(&self.operator.as_str())
    }
}
impl fmt::Display for BinaryOperation {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match write!(fmt, "{} {} {}", self.left, self.operator, self.right) {
            Ok(_) => (),
            Err(e) => return Err(e),
        }
        Ok(())
    }
}
