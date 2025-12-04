use gelato_parser::tokens::{Token, Tokens};
use gelato_tools::nodes::BinaryTreeNode;

use crate::app::{error::Result, statement::{operator::OperatorNode, value::Value}};

pub mod freename;
pub mod function;
pub mod restriction;
pub mod value;
pub mod conditional;
pub mod group;
pub mod operator;
#[derive(Debug, Clone)]
pub struct Program {
    pub(crate) ast: OperatorNode,
}

impl Program {
    pub fn new(tokens: &mut Tokens) -> Result<Program> {
        let mut ast = OperatorNode::parse_next(tokens, false)?;
        ast.bubble();
        Ok(Self { ast })
    }
}