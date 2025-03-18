#[derive(Debug, Clone)]
pub struct Type {
    pub name: String,
    pub typetype: TypeType,
    pub generic_child: Option<Box<Type>>,
}

#[derive(Debug, Clone)]
pub enum TypeType {
    Normal,
    Pointer,
}
