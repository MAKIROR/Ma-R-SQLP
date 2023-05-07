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
    match iter.next() {
        Some(Token::Keyword(Keyword::Where)) => (),
        _  => return Ok(None),
    }

    let condition = parse_condition(iter)?;
    return Ok(Some(condition))
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
            Token::Num(ref s) | Token::Identifier(ref s) => {
                if let Some(op) = operator {
                    let right_node = ASTNode::default(NodeType::Identifier(s.clone()));
                    left_expr = Some(Expression::new(
                        left_expr.take().unwrap().ast, 
                        op,
                        right_node
                    ));
                    operator = None;
                } else {
                    left_expr = Some(Expression::new_with_ast(ASTNode::default(NodeType::Identifier(s.clone()))));
                }
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