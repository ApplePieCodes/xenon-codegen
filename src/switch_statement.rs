use crate::case::Case;

#[derive(Debug, Clone)]
pub struct SwitchStatement {
    pub cases: Vec<Case>,
}
