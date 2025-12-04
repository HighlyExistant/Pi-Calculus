use gelato_parser::{lexer::{FromLexer, Lexer}, tokens::Tokens};

use crate::app::{error::Result, statement::Program};
pub mod statement;
pub mod state;
pub mod error;
/// The applications parsing is divided in parse steps
/// Parse Step 1: Turn the text into tokens.
/// Parse Step 2: Create the AST.
/// Parse Step 3: Fix the AST  using operator precedence.
#[derive(Debug)]
pub struct App {
    lexer: Lexer,
    pub tokens: Tokens,
    pub program: Program,
}

impl App {
    pub fn new(text: String) -> Result<App> {
        let mut lexer = Lexer::new(text);
        let mut tokens = Tokens::from_lexer(&mut lexer).unwrap();
        let program = Program::new(&mut tokens)?;
        Ok(Self { lexer, tokens, program })
    }
}