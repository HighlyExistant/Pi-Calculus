use crate::{lexer::{FromLexer, Lexer}, tokens::Tokens};

mod tokens;
mod lexer;

fn main() {
    let val = "gah a 213, akd )_ (sda)";
    let mut lexer = Lexer::new(val.to_string());
    let val = Tokens::from_lexer(&mut lexer);
    println!("{:#?}", val);
}
