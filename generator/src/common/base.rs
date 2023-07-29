use crate::interface::{Code, Expression};

#[derive(Debug, Copy, Clone)]
pub enum Operation {
    Add,
    Sub,
}

pub struct OperationWithFlag {
    pub result: Expression,
    pub code: Code,
}