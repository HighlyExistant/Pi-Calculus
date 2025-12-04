use gelato_parser::tokens::Punct;
use thiserror::Error;

use crate::app::error;

#[derive(Debug, Error, Clone)]
pub enum PiResult {
    #[error("Unexpected token appeared. Expected {0}")]
    UnexpectedToken(&'static str),
    #[error("Expected statement")]
    ExpectedStatement,
    #[error("Some internal error has ocurred in the program. Please contact the developers.")]
    InternalError,
    #[error("Expected operators '|', '+', '.'")]
    ExpectedOperator,
    #[error("The only literals present in the syntax are '0'")]
    OnlyNil,
    #[error("Recoverable error, use the damn punct...")]
    InGroup(Punct),
}

pub type Result<T> = std::result::Result<T, PiResult>;

pub const IO_PUNCT_ERROR: PiResult = PiResult::UnexpectedToken("'(' or '<'");
pub const IO_CLOSE_PAREN: PiResult = PiResult::UnexpectedToken("')'");
pub const FREENAME_ERROR: PiResult = PiResult::UnexpectedToken("freename");
pub const STATEMENT_ERROR: PiResult = PiResult::UnexpectedToken("statement");