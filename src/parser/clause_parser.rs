use std::{
    vec::IntoIter,
    iter::Peekable,
};
use super::{
    error::{ParseError, Result},
    expression_parser::*,
    super::{
        models::structs::*,
        datatype::{
            token::*,
            keyword::Keyword,
            symbol::Symbol,
        },
    }
};

pub fn parse_where(iter: &mut Peekable<IntoIter<Token>>) -> Result<Option<Condition>> {
    match iter.peek() {
        Some(Token::Keyword(Keyword::Where)) => (),
        _  => return Ok(None),
    }
    iter.next();

    let condition = parse_condition(iter)?;
    return Ok(Some(condition))
}

pub fn parse_having(iter: &mut Peekable<IntoIter<Token>>) -> Result<Option<Condition>> {
    match iter.peek() {
        Some(Token::Keyword(Keyword::Having)) => (),
        _  => return Ok(None),
    }
    iter.next();

    let condition = parse_condition(iter)?;
    return Ok(Some(condition))
}

pub fn parse_projection(iter: &mut Peekable<IntoIter<Token>>) -> Result<Column> {
    let mut column_names = Vec::new();   

    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => column_names.push(name),
            Some(Token::Symbol(Symbol::Asterisk)) => return Ok(Column::AllColumns),
            Some(Token::Symbol(_)) => continue,
            Some(Token::Keyword(_)) => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingToken(Token::Keyword(Keyword::From)))
        }
    }
    return Ok(Column::Columns(column_names));
}

pub fn parse_groupby(iter: &mut Peekable<IntoIter<Token>>) -> Result<Column> {
    match iter.peek() {
        Some(Token::Keyword(Keyword::GroupBy)) => (),
        _  => return Ok(Column::AllColumns),
    }
    iter.next();

    let mut column_names = Vec::new();   
    loop {
        match iter.peek() {
            Some(Token::Identifier(name)) => column_names.push(name.clone()),
            Some(Token::Symbol(Symbol::Asterisk)) => return Ok(Column::AllColumns),
            Some(Token::Symbol(Symbol::Comma)) => {
                iter.next();
                continue;
            },
            Some(Token::Keyword(Keyword::Having)) => break,
            Some(token) if token.is_terminator() => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingColumn)
        }
        iter.next();
    }
    return Ok(Column::Columns(column_names));
}

pub fn parse_orderby(iter: &mut Peekable<IntoIter<Token>>) -> Result<Option<Vec<(String, Sort)>>> {
    match iter.peek() {
        Some(Token::Keyword(Keyword::OrderBy)) => (),
        _  => return Ok(None),
    }
    iter.next();

    let mut order_by: Vec<(String, Sort)> = Vec::new();

    loop {
        match iter.peek() {
            Some(Token::Identifier(name)) => {
                let current_name = name.clone();
                iter.next();

                match iter.next() {
                    Some(t) => {
                        let sort = match t {
                            Token::Keyword(Keyword::Asc) => Sort::ASC,
                            | Token::Keyword(Keyword::Desc) => Sort::DESC,
                            _ => return Err(ParseError::UnexpectedToken(t.clone())),
                        };
                        let tuple = (current_name, sort);
                        order_by.push(tuple);
                    },
                    None => return Err(ParseError::MissingSort)
                }
            },
            Some(Token::Symbol(Symbol::Comma)) => {
                iter.next();
                continue;
            },
            Some(token) if token.is_terminator() => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingColumn)
        }
    }
    return Ok(Some(order_by));
}

pub fn parse_table(iter: &mut Peekable<IntoIter<Token>>) -> Result<String> {
    match iter.next() {
        Some(Token::Identifier(name)) => return Ok(name),
        _ => return Err(ParseError::MissingToken(Token::Identifier("Table name".to_string())))
    }
}