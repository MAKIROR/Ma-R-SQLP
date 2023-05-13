use std::{
    vec::IntoIter,
    iter::Peekable,
};
use super::{
    error::{ParseError, Result},
    datatype::token::*,
    models::{
        ast::*,
        structs::*,
    },
    datatype::{
        keyword::Keyword,
        symbol::Symbol,
    },
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

fn parse_condition(iter: &mut Peekable<IntoIter<Token>>) -> Result<Condition> {
    let mut left: Option<Condition> = None;

    while let Some(token) = iter.peek() {
        match token {
            Token::Keyword(Keyword::And)
            | Token::Keyword(Keyword::Or)
            | Token::Keyword(Keyword::Not) => {
                let current_token = token.clone();
                iter.next();
                let next_condition = parse_condition(iter)?;
                left = match current_token {
                    Token::Keyword(Keyword::And) => {
                        Some(Condition::And {
                            left: Box::new(left.take().unwrap().clone()),
                            right: Box::new(next_condition)
                        })
                    },
                    Token::Keyword(Keyword::Or) => {
                        Some(Condition::Or {
                            left: Box::new(left.take().unwrap().clone()),
                            right: Box::new(next_condition)
                        })
                    },
                    Token::Keyword(Keyword::Not) => Some(Condition::Not(Box::new(next_condition))),
                    _ => return Err(ParseError::UnknownError),
                };
            },
            Token::Symbol(Symbol::LeftParen) => {
                iter.next();
                let next_condition = parse_condition(iter)?;
                if let Some(Token::Symbol(Symbol::RightParen)) = iter.next() {
                    left = Some(next_condition);
                } else {
                    return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen)));
                }
            },
            token if token.is_terminator() => break,
            Token::Symbol(Symbol::RightParen) | Token::Keyword(_) => break,
            Token::Symbol(_) | Token::Number(_) => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            }
            Token::Identifier(_) | Token::Variable(_) | Token::Function(_) => {
                left = Some(parse_comparison(iter)?);
            }
            Token::Comment(_) => (),
        }
    }

    if let Some(r) = left {
        return Ok(r);
    }
    return Err(ParseError::IncorrectCondition);
}

fn parse_comparison(iter: &mut Peekable<IntoIter<Token>>) -> Result<Condition> {
    let left = match iter.peek() {
        Some(Token::Identifier(ref s)) => Value::Identifier(s.clone()),
        Some(Token::Variable(ref v)) => Value::Variable(v.clone()),
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        None => return Err(ParseError::MissingComparator),
    };
    iter.next();
    let operator = match iter.next() {
        Some(Token::Symbol(t)) if t.is_comparator() => t,
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        None => return Err(ParseError::MissingComparator),
    };

    let right = parse_next_term(iter)?;

    Ok(
        Condition::Comparison {
            left,
            operator,
            right,
        }
    )
}

fn parse_next_term(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> {
    let mut left_expr = parse_next_factor(iter)?;

    while let Some(token) = iter.peek() {
        let symbol = match token {
            Token::Symbol(s) => s.clone(),
            _ => break,
        };

        match symbol {
            Symbol::Plus | Symbol::Minus => {
                iter.next();
                let right_expr = parse_next_factor(iter)?;
                left_expr = Expression::new(
                    left_expr.ast,
                    symbol,
                    right_expr.ast,
                );
            }
            _ => break,
        }
    }

    Ok(left_expr)
}

fn parse_next_factor(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> {
    let mut left_expr = parse_primary(iter)?;

    while let Some(token) = iter.peek() {
        let symbol = match token {
            Token::Symbol(s) => s.clone(),
            _ => break,
        };

        match symbol {
            Symbol::Asterisk | Symbol::Slash => {
                iter.next();
                let right_expr = parse_primary(iter)?;
                left_expr = Expression::new(
                    left_expr.ast,
                    symbol,
                    right_expr.ast,
                );
            }
            _ => break,
        }
    }

    Ok(left_expr)
}

fn parse_primary(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> { 
    if let Some(token) = iter.next() {
        match token {
            Token::Identifier(ref s) => return Ok(Expression::new_left(NodeType::Identifier(s.clone()))),
            Token::Number(ref s) => return Ok(Expression::new_left(NodeType::Number(s.clone()))),
            Token::Symbol(Symbol::LeftParen) => {
                let expr = parse_next_term(iter)?;
                match iter.next() {
                    Some(Token::Symbol(Symbol::RightParen)) => Ok(expr),
                    _ => return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen))),
                }
            }
            Token::Symbol(Symbol::Plus) | Token::Symbol(Symbol::Minus) => {
                let expr = parse_primary(iter)?;
                return Ok(Expression::new_unary_op(
                    token.as_symbol().take().unwrap(),
                    expr.ast
                ));
            }
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    } else {
        return Err(ParseError::IncorrectExpression);
    }
}