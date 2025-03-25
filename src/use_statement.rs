use super::identifier::Identifier;

#[derive(Clone, Default)]
pub struct UseStatement {
    pub name: Identifier
}
impl UseStatement {
    pub fn new(name: Identifier) -> UseStatement {
        return UseStatement {
            name
        };
    }
    
    pub fn is_valid(&self) -> bool {
        return self.name.is_valid();
    }
}