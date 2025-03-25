use enum_as_inner::EnumAsInner;

use super::identifier::Identifier;

/// An Expression
#[derive(Clone, Default, EnumAsInner)]
pub enum Expression {
    /// Parentheses
    Parentheses(Parentheses),
    /// Any singular literal
    Literal(Literal),
    /// A unary operation
    UnaryOperation(UnaryOperation),
    /// A binary operation
    BinaryOperation(BinaryOperation),
    #[default]
    Null
}
impl Expression {
    /// Checks if the instance of expression is valid
    pub fn is_valid(&self) -> bool {
        if self.is_parentheses() {
            return self.as_parentheses().unwrap().is_valid();
        }
        else if self.is_literal() {
            return self.as_literal().unwrap().is_valid();
        }
        else if self.is_unary_operation() {
            return self.as_unary_operation().unwrap().is_valid();
        }
        else if self.is_binary_operation() {
            return self.as_binary_operation().unwrap().is_valid();
        }
        else {
            return false;
        }
    }
}

/// A set of parentheses surrounding an expression
#[derive(Clone, Default)]
pub struct Parentheses {
    /// The contained expression
    pub value: Box<Expression>
}
impl Parentheses {
    /// Creates a new valid instance of Parentheses
    pub fn new(value: Expression) -> Parentheses {
        return Parentheses {
            value: Box::new(value)
        };
    }

    /// Checks if the instance is valid
    pub fn is_valid(&self) -> bool {
        return self.value.is_valid();
    }
}

/// A boolean literal
#[derive(Clone, Default)]
pub struct BooleanLiteral {
    /// The value of the literal
    pub value: bool
}
impl BooleanLiteral {
    /// Creates a new valid instance of BooleanLiteral
    pub fn new(value: bool) -> BooleanLiteral {
        return BooleanLiteral {
            value
        };
    }

    /// Checks if the instance is valid
    pub fn is_valid(&self) -> bool {
        return true;
    }
}

#[derive(Clone, Default)]
pub struct IntegerLiteral {
    pub value: i64
}
impl IntegerLiteral {
    pub fn new(value: i64) -> IntegerLiteral {
        return IntegerLiteral {
            value: value
        };
    }

    pub fn is_valid(&self) -> bool {
        return true;
    }
}

#[derive(Clone, Default)]
pub struct FloatLiteral {
    pub value: f64
}
impl FloatLiteral {
    pub fn new(value: f64) -> FloatLiteral {
        return FloatLiteral {
            value
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.value.is_normal();
    }
}

#[derive(Clone, Default)]
pub struct CharLiteral {
    pub value: char
}
impl CharLiteral {
    pub fn new(value: char) -> CharLiteral {
        return CharLiteral {
            value
        };
    }

    pub fn is_valid(&self) -> bool {
        return true;
    }
}

#[derive(Clone, Default)]
pub struct StringLiteral {
    pub value: String
}
impl StringLiteral {
    pub fn new(value: String) -> StringLiteral {
        return StringLiteral {
            value
        };
    }

    pub fn is_valid(&self) -> bool {
        return true;
    }
}

#[derive(Clone, Default, EnumAsInner)]
pub enum Literal {
    BooleanLiteral(BooleanLiteral),
    IntegerLiteral(IntegerLiteral),
    HexLiteral(IntegerLiteral),
    BinaryLiteral(IntegerLiteral),
    FloatLiteral(FloatLiteral),
    CharLiteral(CharLiteral),
    StringLiteral(StringLiteral),
    FunctionCall(Identifier),
    #[default]
    Null
}
impl Literal {
    pub fn is_valid(&self) -> bool {
        if self.is_boolean_literal() {
            return self.as_boolean_literal().unwrap().is_valid();
        }
        else if self.is_integer_literal() {
            return self.as_integer_literal().unwrap().is_valid();
        }
        else if self.is_hex_literal() {
            return self.as_hex_literal().unwrap().is_valid();
        }
        else if self.is_binary_literal() {
            return self.as_binary_literal().unwrap().is_valid();
        }
        else if self.is_float_literal() {
            return self.as_float_literal().unwrap().is_valid();
        }
        else if self.is_char_literal() {
            return self.as_char_literal().unwrap().is_valid();
        }
        else if self.is_string_literal() {
            return self.as_string_literal().unwrap().is_valid();
        }
        else if self.is_function_call() {
            return self.as_function_call().unwrap().is_valid();
        }
        else {
            return false;
        }
    }
}

#[derive(Clone, Default)]
pub struct UnaryOperation {
    pub operator: String,
    pub value: Box<Expression>
}
impl UnaryOperation {
    pub fn new(operator: String, value: Expression) -> UnaryOperation {
        return UnaryOperation {
            operator,
            value: Box::new(value)
        };
    }

    pub fn is_valid(&self) -> bool {
        return ["+", "-", "!", "&", "*"].contains(&self.operator.as_str()) && self.value.is_valid();
    }
}

#[derive(Clone, Default)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: String,
    pub right: Box<Expression>
}
impl BinaryOperation {
    pub fn new(left: Expression, operator: String, right: Expression) -> BinaryOperation {
        return BinaryOperation {
            left: Box::new(left),
            operator,
            right: Box::new(right)
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.left.is_valid() {
            return false;
        }

        if !self.right.is_valid() {
            return false;
        }

        return ["as", "*", "/", "%", "+", "-", "<<", ">>", "&", "^", "|", "==", "!=", "<", "<=", ">", ">=", "&&", "||"].contains(&self.operator.as_str());
    }
}