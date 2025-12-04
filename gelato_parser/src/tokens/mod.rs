mod span;
mod ident;
mod literal;
mod punct;
use std::fmt::Display;

pub use span::*;
pub use ident::*;
pub use literal::*;
pub use punct::*;

use crate::lexer::FromLexer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Ident(Ident),
    Literal(Literal),
    Punct(Punct),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Ident(ident) => {
                f.write_str(format!("{}", ident).as_str())
            }
            Token::Literal(literal) => {
                f.write_str(format!("{}", literal).as_str())
            }
            Token::Punct(punct) => {
                f.write_str(format!("{}", punct).as_str())
            }
        }
    }
}

impl Token {
    pub fn is_literal(&self) -> bool {
        if let Self::Literal(_) = self {
            return true;
        }
        false
    }
    pub fn is_ident(&self) -> bool {
        if let Self::Ident(_) = self {
            return true;
        }
        false
    }
    pub fn is_punct(&self) -> bool {
        if let Self::Punct(_) = self {
            return true;
        }
        false
    }
    pub fn is_punct_subset(&self, c: char) -> bool {
        if let Self::Punct(p) = self {
            return p.punct() == c;
        }
        false
    }
    pub fn get_literal(&self) -> Option<Literal> {
        if let Self::Literal(lit) = self {
            return Some(lit.clone());
        }
        None
    }
    pub fn get_ident(&self) -> Option<Ident> {
        if let Self::Ident(ident) = self {
            return Some(ident.clone());
        }
        None
    }
    pub fn get_punct(&self) -> Option<Punct> {
        if let Self::Punct(punct) = self {
            return Some(punct.clone());
        }
        None
    }
}

impl ContainsSpan for Token {
    fn span(&self) -> &Span {
        match self {
            Token::Ident(ident) => &ident.span(),
            Token::Literal(literal) => &literal.span(),
            Token::Punct(punct) => &punct.span(),
        }
    }
}

impl FromLexer for Token {
    fn from_lexer(lexer: &mut crate::lexer::Lexer) -> Option<Self> {
        lexer.skip_whitespace();
        let get = lexer.get_char()?;
        println!("{get}");
        if get.is_alphabetic() {
            return Some(Self::Ident(Ident::from_lexer(lexer)?));
        }
        if get.is_ascii_punctuation() {
            return Some(Self::Punct(Punct::from_lexer(lexer)?));
        }
        if get.is_ascii_digit() {
            return Some(Self::Literal(Literal::from_lexer(lexer)?));
        }
        None
    }
}
#[derive(Debug, Clone)]
pub struct Tokens {
    tokens: Vec<Token>,
    cursor: usize,
}

impl FromLexer for Tokens {
    fn from_lexer(lexer: &mut crate::lexer::Lexer) -> Option<Self> {
        let mut tokens = vec![];
        while let Some(token) = Token::from_lexer(lexer) {
            tokens.push(token);
        }
        Some(Self::new(tokens))
    }
}

impl Tokens {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }
    /// Assumes the next is ident and returns an option. If
    /// it is not an ident, the program should return a syntax error
    pub fn next_if_ident(&mut self) -> Option<Ident> {
        self.next()?.get_ident()
    }
    /// Assumes the next is punct and returns an option. If
    /// it is not an ident, the program should return a syntax error
    pub fn next_if_punct(&mut self) -> Option<Punct> {
        self.next()?.get_punct()
    }
    pub fn get_puncts(&mut self, count: usize) -> Option<Puncts> {
        let mut puncts = String::new();
        let mut span = Span::new(0..0);
        let mut prev = self.next_if_punct()?;
        puncts.push(prev.punct());
        span.range.start = prev.span().start();
        for i in 0..(count-1) {
            if let Some(punct) = self.next_if_punct() {
                if !prev.are_adjacent(&punct) {
                    return None;
                }
                puncts.push(punct.punct());
                prev = punct;
            } else {
                return None;
            }
        }
        span.range.end = prev.span().end();
        Some(Puncts::new(span, puncts))
    }
    pub fn peek(&self) -> Option<Token> {
        self.tokens.get(self.cursor).cloned()
    }
    pub fn peek_punct(&self) -> Option<Punct> {
        self.peek()?.get_punct()
    }
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let slice = &self.tokens[self.cursor..self.tokens.len()];
        for i in slice {
            f.write_str(format!("{}", i).as_str())?;
        }
        Ok(())
    }
}

impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next) = self.tokens.get(self.cursor) {
            self.cursor += 1;
            Some(next.clone())
        } else {
            None
        }
    }
}