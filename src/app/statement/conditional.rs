use gelato_parser::tokens::{Punct, Tokens};

use crate::app::{error::{PiError, Result}, statement::freename::PiFreename};
#[derive(Debug, Clone)]
pub enum ConditionalOp {
    Eq,
    NEq,
}
#[derive(Debug, Clone)]
pub struct PiConditional {
    open: Punct,
    lhs: PiFreename,
    eq: ConditionalOp,
    rhs: PiFreename,
}

impl PiConditional {
    pub fn parse_next(tokens: &mut Tokens, open: Punct) -> Result<Self> {
        let lhs = PiFreename::parse_next(tokens)?;
        let parse = tokens.get_puncts(2).ok_or(PiError::UnexpectedToken("'==' or '!='"))?;
        let cond_op = match parse.puncts() {
            "==" => {
                ConditionalOp::Eq
            }
            "!=" => {
                ConditionalOp::NEq
            }
            _ => return Err(PiError::UnexpectedToken("'==' or '!='")),
        };
        let rhs = PiFreename::parse_next(tokens)?;
        let close = tokens.next_if_punct().ok_or(PiError::UnexpectedToken("']'"))?;
        if close.punct() != ']' {
            return Err(PiError::UnexpectedToken("']'"));
        }
        Ok(Self { open, lhs, eq: cond_op, rhs })
    }
}