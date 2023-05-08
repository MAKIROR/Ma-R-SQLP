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
            Token::Keyword(_) | Token::Symbol(_) | Token::Num(_) => {
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
    let mut left_expr: Option<Expression> = None;
    let mut operator: Option<Symbol> = None;

    while let Some(token) = iter.peek() {
        if left_expr.is_some() && operator.is_none() && !token.is_operator() {
            break;
        }
        match token {
            Token::Num(ref s) => {
                if let Some(op) = operator {
                    let right_node = ASTNode::default(NodeType::Number(s.clone()));
                    left_expr = Some(Expression::new(
                        left_expr.take().unwrap().ast, 
                        op,
                        right_node
                    ));
                    operator = None;
                } else {
                    left_expr = Some(Expression::new_with_ast(ASTNode::default(NodeType::Number(s.clone()))));
                }
            }
            Token::Identifier(ref s) => {
                if let Some(op) = operator {
                    let right_node = ASTNode::default(NodeType::Identifier(s.clone()));
                    left_expr = Some(Expression::new(
                        left_expr.take().unwrap().ast, 
                        op,
                        right_node
                    ));
                } else {
                    left_expr = Some(Expression::new_with_ast(ASTNode::default(NodeType::Identifier(s.clone()))));
                }
                return Ok(left_expr.take().unwrap());
            }
            Token::Symbol(Symbol::LeftParen) => {
                let next_expr = parse_next_expression(iter)?;
                if let Some(op) = operator {
                    let right_expr = next_expr;
                    left_expr = Some(Expression::new(
                        left_expr.take().unwrap().ast,
                        op,
                        right_expr.ast
                    ));
                    operator = None;
                } else {
                    left_expr = Some(next_expr);
                }
                match iter.next() {
                    Some(Token::Symbol(Symbol::RightParen)) => (),
                    Some(t) => return Err(ParseError::UnexpectedToken(t)),
                    None => return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen))),
                }
            }
            Token::Symbol(Symbol::Semicolon)
            | Token::Symbol(Symbol::RightParen)
            | Token::Keyword(_) => {
                if let Some(n) = left_expr {
                    if operator.is_none() {
                        return Ok(n);
                    }
                }
                return Err(ParseError::IncorrectExpression);
            }
            Token::Symbol(ref s) => {
                if !s.is_operator() || operator.is_some() {
                    return Err(ParseError::UnexpectedToken(token.clone()));
                }
                operator = Some(s.clone());
            }
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
        iter.next();
    }

    if let Some(n) = left_expr {
        if operator.is_none() {
            return Ok(n);
        }
    }
    return Err(ParseError::IncorrectExpression);
}