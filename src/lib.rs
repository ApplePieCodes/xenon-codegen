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
#[derive(Debug, Clone)]
pub enum Visibility {
    Public,
    Private,
}

#[cfg(test)]
mod tests {
    use crate::attribute::Attribute;

    #[test]
    fn attribute_test() {
        let attribute = Attribute::new("attribute");
        assert!(attribute.to_string() == "#[attribute]\n");
    }
    #[test]
    fn attribute_test_with_value() {
        let mut attribute = Attribute::new("attribute");
        attribute.value = Some(String::from("value"));
        assert!(attribute.to_string() == "#[attribute(value)]\n");
    }
}
