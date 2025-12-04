use gelato_parser::tokens::{Punct, Token, Tokens};

use crate::app::{error::{Result, STATEMENT_ERROR}, statement::{Value, operator::OperatorNode, value::Values}};
#[derive(Debug, Clone)]
pub struct PiGroup {
    statements: OperatorNode,
}

impl PiGroup {
    pub fn parse_next(tokens: &mut Tokens, open: &Punct, first_token: Token) -> Result<Self> {
        if let Some(close) = first_token.get_punct() { // break statement
            if close.punct() == ')' {
                return Ok(PiGroup {  statements: OperatorNode::nil() });
            }
        }
        let operator = OperatorNode::parse_next_prev(tokens, first_token, true)?;
        Ok(Self { statements: operator })
    }
}