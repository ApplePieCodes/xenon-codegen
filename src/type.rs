use super::identifier::Identifier;

#[derive(Clone, Default)]
pub struct Type {
    pub name: Identifier,
    pub generic_child: Option<Box<Type>>
}
impl Type {
    pub fn new(name: Identifier) -> Type {
        return Type {
            name,
            generic_child: None
        };
    }

    pub fn is_valid(&self) -> bool {
        if !self.name.is_valid() {
            return false;
        }

        match self.generic_child.as_ref() {
            Some(c) => {
                if !c.is_valid() {
                    return false;
                }
            }
            None => {
                return true;
            }
        }

        return true;
    }
}