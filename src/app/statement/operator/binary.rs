use std::ops::{Deref, DerefMut};

use gelato_tools::nodes::BinaryTreeNode;

use crate::app::statement::operator::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinOp {
    Concurrency,
    Choice,
    Sequential,
}

impl BinOp {
    pub fn precedence(&self) -> usize {
        match self {
            BinOp::Choice => 1,
            BinOp::Concurrency => 1,
            BinOp::Sequential => 0,
        }
    }
}