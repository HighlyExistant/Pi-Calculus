use thiserror::Error;

#[derive(Debug, Error, Clone, Copy)]
pub enum PiError {
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
}

pub type Result<T> = std::result::Result<T, PiError>;

pub const IO_PUNCT_ERROR: PiError = PiError::UnexpectedToken("'(' or '<'");
pub const IO_CLOSE_PAREN: PiError = PiError::UnexpectedToken("')'");
pub const FREENAME_ERROR: PiError = PiError::UnexpectedToken("freename");
pub const STATEMENT_ERROR: PiError = PiError::UnexpectedToken("statement");