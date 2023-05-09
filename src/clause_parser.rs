use std::{
    vec::IntoIter,
    iter::Peekable,
    collections::VecDeque
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
    match iter.next() {
        Some(Token::Keyword(Keyword::Where)) => (),
        _  => return Ok(None),
    }

    let condition = parse_condition(iter)?;
    return Ok(Some(condition))
}

pub fn parse_projection(iter: &mut Peekable<IntoIter<Token>>) -> Result<Projection> {
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

pub fn parse_table(iter: &mut Peekable<IntoIter<Token>>) -> Result<String> {
    match iter.next() {
        Some(Token::Identifier(name)) => return Ok(name),
        _ => return Err(ParseError::MissingToken(Token::Identifier("Table name".to_string())))
    }
}

fn parse_columns(iter: &mut Peekable<IntoIter<Token>>) -> Result<Vec<String>> {
    let mut paren = VecDeque::new();
    let mut values: Vec<String> = Vec::new();
    if let Some(Token::Symbol(Symbol::LeftParen)) = &iter.peek() {
        paren.push_back(Symbol::LeftParen);
        iter.next();
    };
    
    loop {
        match iter.next() {
            Some(Token::Identifier(value)) | Some(Token::Number(value)) => values.push(value),
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
            Token::Symbol(Symbol::Semicolon) | Token::Symbol(Symbol::RightParen) => break,
            Token::Keyword(_) | Token::Symbol(_) | Token::Number(_) => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            }
            Token::Identifier(_) => {
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
        Some(Token::Identifier(ref s)) => s.clone(),
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        None => return Err(ParseError::MissingComparator),
    };
    iter.next();
    let operator = match iter.next() {
        Some(Token::Symbol(t)) if t.is_comparator() => t,
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        None => return Err(ParseError::MissingComparator),
    };

    let right = parse_next_expression(iter)?;

    Ok(
        Condition::Comparison {
            left,
            operator,
            right,
        }
    )
}

fn parse_next_expression(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> {
    parse_next_term(iter)
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
                let expr = parse_next_expression(iter)?;
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