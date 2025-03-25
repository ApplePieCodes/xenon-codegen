use crate::expression::Expression;

#[derive(Clone, Default)]
pub struct Identifier {
    /// The name of the current identifier
    pub name: String,
    /// The next identifier after this one
    pub child: Option<Box<Identifier>>,
    /// If none, this is not a function call
    pub arguments: Option<Vec<Expression>>
}
impl Identifier {
    pub fn new(name: String) -> Identifier {
        return Identifier {
            name,
            child: None,
            arguments: None
        };
    }

    pub fn is_valid(&self) -> bool {
        if self.name.is_empty() {
            return false;
        }
        match self.child.as_ref() {
            Some(c) => {
                if !c.is_valid() {
                    return false;
                }
            }
            None => ()
        }
        match self.arguments.as_ref() {
            Some(a) => {
                for i in 0..a.len() {
                    if !a[i].is_valid() {
                        return false;
                    }
                }
                return true;
            }
            None => {
                return true;
            }
        }
    }
}