use thiserror::Error;
use super::datatype::token::*;


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: '{0}'")]
    UnexpectedToken(Token),
    #[error("Missing token: '{0}'")]
    MissingToken(Token),
}

pub type Result<T> = std::result::Result<T, ParseError>;