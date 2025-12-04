use crate::tokens::Tokens;

pub trait FromTokens: Sized {
    type Error;
    fn from_tokens(tokens: &mut Tokens) -> Result<Self, Self::Error>;
}