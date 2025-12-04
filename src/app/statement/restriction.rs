use gelato_parser::tokens::{Punct, Tokens};

use crate::app::{error::{FREENAME_ERROR, IO_CLOSE_PAREN, IO_PUNCT_ERROR, Result}, statement::freename::PiFreename};

#[derive(Debug, Clone)]
pub struct PiRestriction {
    open: Punct,
    local: PiFreename,
    close: Punct,
}

impl PiRestriction {
    pub fn parse_next(tokens: &mut Tokens, open: &Punct) -> Result<Self> {
        let freename = PiFreename::parse_next(tokens)?;
        let close = tokens.next_if_punct().ok_or(IO_CLOSE_PAREN)?;
        Ok(Self { open: open.clone(), local: freename, close })
    }
}