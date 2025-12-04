use gelato_parser::{lexer::{FromLexer, Lexer}, tokens::Tokens};
pub mod statement;
pub mod error;
/// The applications parsing is divided in parse steps
/// Parse Step 1: Turn the text into tokens.
/// Parse Step 2: Create the AST.
/// Parse Step 3: Fix the AST  using operator precedence.
#[derive(Debug)]
pub struct App {
    lexer: Lexer,
    pub tokens: Tokens,
}

impl App {
    pub fn new(text: String) -> Self {
        let mut lexer = Lexer::new(text);
        let tokens = Tokens::from_lexer(&mut lexer).unwrap();
        Self { lexer, tokens }
    }
}