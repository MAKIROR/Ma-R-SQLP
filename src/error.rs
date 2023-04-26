use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParseError {
}

pub type Result<T> = std::result::Result<T, ParseError>;