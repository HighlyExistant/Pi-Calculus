use gelato_parser::tokens::{Punct, Tokens};

use crate::app::{error::{PiResult, Result}, statement::freename::PiFreename};
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
        let parse = tokens.get_puncts(2).ok_or(PiResult::UnexpectedToken("'==' or '!='"))?;
        let cond_op = match parse.puncts() {
            "==" => {
                ConditionalOp::Eq
            }
            "!=" => {
                ConditionalOp::NEq
            }
            _ => return Err(PiResult::UnexpectedToken("'==' or '!='")),
        };
        let rhs = PiFreename::parse_next(tokens)?;
        let close = tokens.next_if_punct().ok_or(PiResult::UnexpectedToken("']'"))?;
        if close.punct() != ']' {
            return Err(PiResult::UnexpectedToken("']'"));
        }
        Ok(Self { open, lhs, eq: cond_op, rhs })
    }
}