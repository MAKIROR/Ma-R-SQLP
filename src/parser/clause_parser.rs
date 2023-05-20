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
    if let Some(Token::Symbol(Symbol::Asterisk)) = iter.peek() {
        return Ok(Column::AllColumns);
    }

    parse_columns(iter)
}

pub fn parse_groupby(iter: &mut Peekable<IntoIter<Token>>) -> Result<Column> {
    match iter.peek() {
        Some(Token::Keyword(Keyword::GroupBy)) => (),
        _  => return Ok(Column::AllColumns),
    }
    iter.next();

    parse_columns(iter)
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

pub fn parse_tables(iter: &mut Peekable<IntoIter<Token>>) -> Result<Vec<(Expression, Option<Expression>)>> {
    match iter.peek() {
        Some(Token::Keyword(Keyword::From)) => (),
        _  => return Err(ParseError::MissingToken(Token::Keyword(Keyword::From))),
    }
    iter.next();

    let tables = parse_items_with_alias(iter)?;

    if tables.len() == 0 {
        return Err(ParseError::MissingTable);
    }
    Ok(tables)
}

fn parse_columns(iter: &mut Peekable<IntoIter<Token>>) -> Result<Column> {
    Ok(Column::Columns(parse_items_with_alias(iter)?))
}

fn parse_items_with_alias(
    iter: &mut Peekable<IntoIter<Token>>
) -> Result<Vec<(Expression, Option<Expression>)>> 
{

    let mut columns = Vec::new();

    loop {
        match iter.peek() {
            Some(Token::Keyword(k)) if k.is_clause() => break,
            _ => ()
        }
        match parse_expression(iter) {
            Ok(e) => {
                let mut alias = None;
                if let Some(Token::Keyword(Keyword::As)) = iter.peek() {
                    iter.next();
                    alias = Some(parse_expression(iter)?);
                }
                columns.push((e, alias));
                if let Some(Token::Symbol(Symbol::Comma)) = iter.peek() {
                    iter.next();
                }
            },
            Err(e) => return Err(e),
        }
    }

    return Ok(columns);
}