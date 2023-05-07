use std::{
    vec::IntoIter,
    iter::Peekable,
    collections::VecDeque
};
use super::{
    clause_parser::*,
    error::{ParseError, Result},
    datatype::token::*,
    models::{
        ast::*,
        structs::*,
    },
    datatype::{
        keyword::Keyword,
        symbol::Symbol,
        arg::Arg,
    },
};

pub fn parse_select(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();
    while let Some(t) = iter.peek() {
        match t {
            Token::Keyword(Keyword::Where) => {
                println!("{:?}", parse_where(&mut iter)?);
            },
            _ => {iter.next();},
        };
    }
    return Ok(ASTNode::default(NodeType::Select));
}

pub fn parse_insert(t: &Vec<Token>) -> Result<ASTNode> {
   todo!()
   // TODO:
}

pub fn parse_delete(t: &Vec<Token>) -> Result<ASTNode> {
   todo!()
   // TODO:
}

fn parse_column(iter: &mut Peekable<IntoIter<Token>>) -> Result<Vec<String>> {
    let mut paren = VecDeque::new();
    let mut values: Vec<String> = Vec::new();
    if let Some(Token::Symbol(Symbol::LeftParen)) = &iter.peek() {
        paren.push_back(Symbol::LeftParen);
        iter.next();
    };
    
    loop {
        match iter.next() {
            Some(Token::Identifier(value)) | Some(Token::Num(value)) => values.push(value),
            Some(Token::Symbol(Symbol::RightParen)) => {
                if Some(Symbol::LeftParen) == paren.pop_back() {
                    break;
                }
                return Err(ParseError::UnexpectedToken(Token::Symbol(Symbol::RightParen)))
            },
            Some(Token::Symbol(_)) => continue,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => {
                if Some(Symbol::LeftParen) == paren.pop_back() {
                    return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen)))
                }
                break;
            }
        }
    }
    Ok(values)
}

fn parse_projection(iter: &mut Peekable<IntoIter<Token>>) -> Result<Projection> {
    let mut column_names = Vec::new();   

    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => column_names.push(name),
            Some(Token::Symbol(Symbol::Asterisk)) => return Ok(Projection::AllColumns),
            Some(Token::Symbol(_)) => continue,
            Some(Token::Keyword(Keyword::From)) => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingToken(Token::Keyword(Keyword::From)))
        }
    }
    return Ok(Projection::Columns(column_names));
}

fn parse_table(iter: &mut Peekable<IntoIter<Token>>) -> Result<String> {
    match iter.next() {
        Some(Token::Identifier(name)) => return Ok(name),
        _ => return Err(ParseError::MissingToken(Token::Identifier("Table name".to_string())))
    }
}

fn parse_optional_args_or(
    iter: &mut Peekable<IntoIter<Token>>,
    args: Vec<Arg>,
    default: Arg,
) -> Arg {
    if let Some(Token::Keyword(keyword)) = iter.peek() {
        if let Some(arg) = Option::from(keyword) {
            if let Some(nodetype) = args.iter().find(|&a| a.clone() == arg) {
                return nodetype.clone();
            }
        }
    }
    default
}

fn match_token(value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(_) => Ok(()),
        None => return Err(ParseError::MissingToken(expect))
    }
}