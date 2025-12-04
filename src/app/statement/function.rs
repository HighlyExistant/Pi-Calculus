use gelato_parser::tokens::{Ident, Punct, Token, Tokens};

use crate::app::{error::{IO_PUNCT_ERROR, PiError, Result}, statement::freename::PiFreename};
#[derive(Debug, Clone)]
pub struct PiFunction {
    ident: PiFreename,
    open: Punct,
    variable: PiFreename,
    close: Punct,
}

impl PiFunction {
    pub fn parse_next(tokens: &mut Tokens, ident: &Ident) -> Result<Self> {
        // expecting either '(' or '<'
        let open = tokens.next_if_punct().ok_or(IO_PUNCT_ERROR)?;
        match open.punct() {
            '(' => {
                Self::match_lparen(tokens, ident, &open)
            }
            '<' => {
                Self::match_larrow(tokens, ident, &open)
            }
            _ => Err(IO_PUNCT_ERROR)
        }
    }
    pub fn match_lparen(tokens: &mut Tokens, ident: &Ident, open: &Punct) -> Result<Self> {
        let variable = PiFreename::parse_next(tokens)?;
        let close = tokens
            .next_if_punct()
            .ok_or(PiError::UnexpectedToken(")"))?;
        if close.punct() != ')' {
            return Err(PiError::UnexpectedToken(")"));
        }
        Ok(Self { ident: PiFreename::from(ident.clone()), open: open.clone(), variable, close })
    }
    pub fn match_larrow(tokens: &mut Tokens, ident: &Ident, open: &Punct) -> Result<Self> {
        let variable = PiFreename::parse_next(tokens)?;
        let close = tokens
            .next_if_punct()
            .ok_or(PiError::UnexpectedToken(">"))?;
        if close.punct() != '>' {
            return Err(PiError::UnexpectedToken(">"));
        }
        Ok(Self { ident: PiFreename::from(ident.clone()), open: open.clone(), variable, close })
    }
}