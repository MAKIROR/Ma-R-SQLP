use thiserror::Error;
use super::datatype::token::*;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: '{0}'")]
    UnexpectedToken(Token),

    #[error("Missing token: '{0}'")]
    MissingToken(Token),

    #[error("Missing comparator")]
    MissingComparator,

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Incorrect number of values: expect {0}")]
    IncorrectValueCount(usize),

    #[error("Incorrect expression")]
    IncorrectExpression,

    #[error("Incorrect condition")]
    IncorrectCondition,

    #[error("Unknown error")]
    UnknownError,
}

pub type Result<T> = std::result::Result<T, ParseError>;