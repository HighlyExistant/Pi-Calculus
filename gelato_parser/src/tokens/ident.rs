use std::{fmt::Display, hash::Hash};

use crate::{lexer::FromLexer, tokens::{ContainsSpan, Span}};

#[derive(Debug, Clone)]
pub struct Ident {
    span: Span,
    ident: String,
}
impl Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.ident.hash(state);
    }
}
impl Eq for Ident {
    
}
impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.ident() == other.ident()
    }
}

impl Ident {
    pub fn new(span: Span, ident: String) -> Self {
        Self { span, ident }
    }
    pub fn ident(&self) -> &str {
        &self.ident
    }
}
impl ContainsSpan for Ident {
    fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.ident))
    }
}

impl FromLexer for Ident {
    fn from_lexer(lexer: &mut crate::lexer::Lexer) -> Option<Self> {
        lexer.skip_whitespace();
        let start = lexer.cursor();

        let mut ident = String::new();
        while let Some(c) = lexer.get_char() {
            if c.is_alphabetic() {
                ident.push(c);
                lexer.shift_cursor();
            } else {
                break;
            }
        }

        let end = lexer.cursor();

        Some(Ident::new(Span::new(start..end), ident))
    }
}