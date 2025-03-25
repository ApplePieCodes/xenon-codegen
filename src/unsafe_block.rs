use super::block::Block;

#[derive(Clone, Default)]
pub struct UnsafeBlock {
    pub body: Block   
}
impl UnsafeBlock {
    pub fn new(body: Block) -> UnsafeBlock {
        return UnsafeBlock {
            body
        };
    }

    pub fn is_valid(&self) -> bool {
        return self.body.is_valid();
    }
}