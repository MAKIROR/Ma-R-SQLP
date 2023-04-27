use super::{
    error::{ParseError, Result},
    datatype::token::*,
    models::{
        ast::*,
        structs::NodeType,
    },
    datatype::{
        keyword::Keyword,
        symbol::Symbol,
    },
};

pub fn parse_select(t: &Vec<Token>) -> Result<ASTNode> {
    let mut tokens = t.clone();
    tokens.remove(0);

    let mut iter = tokens.into_iter();
    let mut column_names = Vec::new();    
    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => column_names.push(name),
            Some(Token::Symbol(_)) => continue,
            Some(Token::Keyword(Keyword::From)) => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingToken(Token::Keyword(Keyword::From)))
        }
    }
    todo!()
}