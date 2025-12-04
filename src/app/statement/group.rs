use gelato_parser::tokens::{Punct, Token, Tokens};

use crate::app::{error::{Result, STATEMENT_ERROR}, statement::Value};
#[derive(Debug, Clone)]
pub struct PiGroup {
    open: Punct,
    statements: Vec<Value>,
    close: Punct,
}

impl PiGroup {
    pub fn parse_next(tokens: &mut Tokens, open: &Punct, first_token: Token) -> Result<Self> {
        let mut statements = vec![];
        if let Some(close) = first_token.get_punct() { // break statement
            if close.punct() == ')' {
                return Ok(PiGroup { open: open.clone(), statements, close });
            }
        }
        let statement = Value::parse_next_start(tokens, first_token)?;
        statements.push(statement);
        while let Some(next) = tokens.next() {
            println!(":::: {next:#?}");
            if let Some(close) = next.get_punct() { // break statement
                if close.punct() == ')' {
                    return Ok(PiGroup { open: open.clone(), statements, close });
                }
            }
            let statement = Value::parse_next_start(tokens, next)?;
            statements.push(statement);
        }
        Err(STATEMENT_ERROR)
    }
}