use std::{fmt::Display, hash::Hash};

use crate::{lexer::FromLexer, tokens::{ContainsSpan, Span}};

#[derive(Debug, Clone)]
pub struct Literal {
    span: Span,
    literal: String,
}
impl Eq for Literal {
    
}
impl Hash for Literal {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.literal.hash(state);
    }
}
impl PartialEq for Literal {
    fn eq(&self, other: &Self) -> bool {
        self.literal() == other.literal()
    }
}

impl Literal {
    pub fn new(span: Span, literal: String) -> Self {
        Self { span, literal }
    }
    pub fn literal(&self) -> &str {
        &self.literal
    }
}
impl ContainsSpan for Literal {
    fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.literal))
    }
}

impl FromLexer for Literal {
    fn from_lexer(lexer: &mut crate::lexer::Lexer) -> Option<Self> {
        lexer.skip_whitespace();
        let start = lexer.cursor();

        let mut ident = String::new();
        while let Some(c) = lexer.get_char() {
            if c.is_numeric() {
                ident.push(c);
                lexer.shift_cursor();
            } else {
                break;
            }
        }

        let end = lexer.cursor();

        Some(Literal::new(Span::new(start..end), ident))
    }
}