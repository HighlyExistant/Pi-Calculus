use std::{fmt::Display, hash::Hash};

use crate::{lexer::FromLexer, tokens::{ContainsSpan, Span}};

#[derive(Debug, Clone)]
pub struct Punct {
    span: Span,
    punct: char,
}
impl Eq for Punct {
    
}
impl Hash for Punct {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.punct.hash(state);
    }
}
impl PartialEq for Punct {
    fn eq(&self, other: &Self) -> bool {
        self.punct() == other.punct()
    }
}

impl Punct {
    pub fn new(span: Span, punct: char) -> Self {
        Self { span, punct }
    }
    pub fn punct(&self) -> char {
        self.punct
    }
}

impl ContainsSpan for Punct {
    fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Punct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.punct))
    }
}
impl FromLexer for Punct {
    fn from_lexer(lexer: &mut crate::lexer::Lexer) -> Option<Self> {
        lexer.skip_whitespace();
        let start = lexer.cursor();

        let char = lexer.next_char()?;

        let end = lexer.cursor();

        Some(Punct::new(Span::new(start..end), char))
    }
}

pub struct Puncts {
    span: Span,
    punct: String,
}
impl Eq for Puncts {
    
}
impl Hash for Puncts {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.punct.hash(state);
    }
}
impl PartialEq for Puncts {
    fn eq(&self, other: &Self) -> bool {
        self.puncts() == other.puncts()
    }
}

impl Puncts {
    pub fn new(span: Span, punct: String) -> Self {
        Self { span, punct }
    }
    pub fn puncts(&self) -> &str {
        self.punct.as_str()
    }
}

impl ContainsSpan for Puncts {
    fn span(&self) -> &Span {
        &self.span
    }
}

impl Display for Puncts {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.punct))
    }
}