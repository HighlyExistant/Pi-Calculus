use gelato_parser::tokens::{Token, Tokens};

use crate::app::{error::Result, statement::{operator::OperatorNode, value::{Value, Values}}};

pub struct Expressions {
    expression: OperatorNode,
}