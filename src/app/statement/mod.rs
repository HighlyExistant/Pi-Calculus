use gelato_parser::tokens::{Token, Tokens};
use gelato_tools::nodes::BinaryTreeNode;

use crate::app::{statement::{value::Value}};

pub mod freename;
pub mod function;
pub mod restriction;
pub mod value;
pub mod conditional;
pub mod group;
pub mod operator;
pub mod expressions;

pub struct Program {
}
