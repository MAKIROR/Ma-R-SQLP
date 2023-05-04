use thiserror::Error;
use super::{
    datatype::token::*,
    structs::Expression
};


#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Unexpected token: '{0}'")]
    UnexpectedToken(Token),

    #[error("Missing token: '{0}'")]
    MissingToken(Token),

    #[error("Syntax error: {0}")]
    SyntaxError(String),

    #[error("Incorrect number of values: expect {0}")]
    IncorrectValueCount(usize),

    #[error("Incorrect Expression: {0}")]
    IncorrectExpression(Expression)
}

pub type Result<T> = std::result::Result<T, ParseError>;