use std::ops::{Deref, DerefMut};

use gelato_tools::nodes::BinaryTreeNode;

use crate::app::statement::operator::Operator;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UniOp {
    Replication,
}

impl UniOp {
    pub fn precedence(&self) -> usize {
        match self {
            UniOp::Replication => 0,
        }
    }
}