use gelato_parser::tokens::{Ident, Literal, Punct, Token, Tokens};

use crate::app::{error::{FREENAME_ERROR, PiResult, Result, STATEMENT_ERROR}, statement::{conditional::PiConditional, function::PiFunction, group::PiGroup, restriction::PiRestriction}};

#[derive(Debug, Clone)]
pub enum Value {
    /// <free-name> <del> <free-name> <del>
    Function(PiFunction),
    /// <lparen> <var> <free-name> <rparen>
    Restriction(PiRestriction),
    /// <lparen> <statement> <rparen>
    /// If the next statement is not [var], then assume it is a
    /// statement.
    Group(PiGroup),
    /// <lsquare> <free-name> <eq> <free-name> <rsquare>
    Conditional(PiConditional),
    /// 0
    Nil,
    Debug(usize),
}

impl Value {
    pub fn parse_next(tokens: &mut Tokens) -> Result<Self> {
        let next = tokens.next().ok_or(PiResult::ExpectedStatement)?;
        Self::parse_next_start(tokens, next)
    }
    pub fn parse_next_start(tokens: &mut Tokens, next: Token) -> Result<Self> {
        let value = match next {
            Token::Ident(ident) => {
                Self::match_ident(tokens, &ident)?
            }
            Token::Literal(literal) => {
                Self::match_literal(tokens, &literal)?
            }
            Token::Punct(punct) => {
                Self::match_punct(tokens, &punct)?
            }
        };
        
        Ok(value)
    }
    fn match_ident(tokens: &mut Tokens, ident: &Ident) -> Result<Self> {
        match ident.ident() {
            _ => {
                let func = PiFunction::parse_next(tokens, ident)?;
                Ok(Self::Function(func))
            }
        }
    }
    fn match_literal(tokens: &mut Tokens, ident: &Literal) -> Result<Self> {
        match ident.literal() {
            "0" => Ok(Self::Nil),
            _ => Err(PiResult::OnlyNil)
        }
    }
    /// Could either be restriction, group or conditional.
    fn match_punct(tokens: &mut Tokens, ident: &Punct) -> Result<Self> {
        match ident.punct() {
            '(' => { // can be either restriction or group
                Self::match_punct_lparen(tokens, ident)
            }
            '[' => { // conditional
                Ok(Self::Conditional(PiConditional::parse_next(tokens, ident.clone())?))
            }
            _ => {
                return Err(PiResult::UnexpectedToken("'(' or '['"));
            }
        }
    }
    fn match_punct_lparen(tokens: &mut Tokens, open: &Punct) -> Result<Self> {
        let freename = tokens.next().ok_or(STATEMENT_ERROR)?;
        match freename.to_string().as_str() {
            "var" => { // restriction
                let restriction = PiRestriction::parse_next(tokens, open)?;
                Ok(Self::Restriction(restriction))
            }
            _ => { // group
                let group = PiGroup::parse_next(tokens, open, freename)?;
                Ok(Self::Group(group))
            }
        }
    }
}
#[derive(Debug, Clone)]
pub struct Values {
    pub(crate) values: Vec<Value>,
}

impl Values {
    pub fn new() -> Self {
        Self { values: vec![] }
    }
    pub fn push_value(&mut self, value: Value) {
        self.values.push(value);
    }
}
