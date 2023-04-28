use thiserror::Error;
use super::datatype::token::*;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: '{0}'")]
    UnexpectedToken(Token),
    #[error("Missing token: '{0}'")]
    MissingToken(Token),
    #[error("Syntax error: {0}")]
    SyntaxError(String),
}

pub type Result<T> = std::result::Result<T, ParseError>;