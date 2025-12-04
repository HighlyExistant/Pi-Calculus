use gelato_parser::tokens::{Ident, Tokens};

use crate::app::error::{PiError, Result};

#[derive(Debug, Clone)]
pub struct PiFreename {
    pub ident: Ident,
}
impl From<Ident> for PiFreename {
    fn from(value: Ident) -> Self {
        Self { ident: value }
    }
}
impl PiFreename {
    pub fn parse_next(tokens: &mut Tokens) -> Result<Self> {
        let ident = tokens
            .next_if_ident()
            .ok_or(PiError::UnexpectedToken("freename"))?;
        Ok(Self { ident })
    }
}