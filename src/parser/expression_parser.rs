use std::{
    vec::IntoIter,
    iter::Peekable,
};
use super::{
    error::{ParseError, Result},
    super::{
        models::{
            ast::*,
            structs::*,
        },
        datatype::{
            token::*,
            keyword::Keyword,
            symbol::Symbol,
        },
    }
};

pub fn parse_condition(iter: &mut Peekable<IntoIter<Token>>) -> Result<Condition> {
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
            Token::Identifier(_) | Token::Variable(_) | Token::Function(_) | Token::Bool(_) => {
                left = Some(parse_comparison(iter)?);
            }
            t => return Err(ParseError::UnexpectedToken(t.clone())),
        }
    }

    if let Some(r) = left {
        return Ok(r);
    }
    return Err(ParseError::IncorrectCondition);
}

fn parse_comparison(iter: &mut Peekable<IntoIter<Token>>) -> Result<Condition> {
    let left = match iter.peek() {
        Some(Token::Identifier(_))
        | Some(Token::Symbol(Symbol::LeftParen))
        | Some(Token::Variable(_)) 
        | Some(Token::Function(_)) => parse_next_term(iter)?,
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        None => return Err(ParseError::MissingComparator),
    };

    let operator = match iter.peek() {
        Some(Token::Symbol(t)) if t.is_comparator() => t.clone(),
        Some(Token::Symbol(Symbol::LeftParen)) => Symbol::LeftParen,
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        None => return Err(ParseError::MissingComparator),
    };
    iter.next();

    let right = parse_next_term(iter)?;

    Ok(
        Condition::Comparison {
            left,
            operator,
            right,
        }
    )
}

pub fn parse_expression(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> {
    let mut left_expr = parse_next_term(iter)?;

    while let Some(token) = iter.peek() {
        let symbol = match token {
            Token::Symbol(s) => s.clone(),
            _ => break,
        };


        match symbol {
            Symbol::Plus | Symbol::Minus => {
                iter.next();
                let right_expr = parse_next_term(iter)?;
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

fn parse_next_term(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> {
    let mut left_expr = parse_factor(iter)?;

    while let Some(token) = iter.peek() {
        let symbol = match token {
            Token::Symbol(s) => s.clone(),
            _ => break,
        };

        match symbol {
            Symbol::Asterisk | Symbol::Slash => {
                iter.next();
                let right_expr = parse_factor(iter)?;
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

fn parse_factor(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> { 
    if let Some(token) = iter.peek() {
        let result = match token {
            Token::Identifier(ref s) => Ok(Expression::new_left(NodeType::Value(Value::Identifier(s.clone())))),
            Token::Number(ref s) => Ok(Expression::new_left(NodeType::Value(Value::Number(s.clone())))),
            Token::Variable(ref v) => Ok(Expression::new_left(NodeType::Value(Value::Variable(v.clone())))),
            Token::Function(_) => {
                let function = parse_function(iter)?;
                return Ok(Expression::new_left(NodeType::Function(Box::new(function))));
            },
            Token::Symbol(Symbol::LeftParen) => {
                iter.next();
                let expr = parse_expression(iter)?;
                return match iter.next() {
                    Some(Token::Symbol(Symbol::RightParen)) => Ok(expr),
                    _ => Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen))),
                };
            }
            Token::Symbol(Symbol::Plus) | Token::Symbol(Symbol::Minus) => {
                let t = token.clone();
                iter.next();
                let expr = parse_factor(iter)?;
                return Ok(Expression::new_unary_op(
                    t.as_symbol().take().unwrap(),
                    expr.ast
                ));
            }
            _ => Err(ParseError::UnexpectedToken(token.clone())),
        };

        iter.next();
        return result;
    } else {
        return Err(ParseError::IncorrectExpression);
    }
}

fn parse_function(iter: &mut Peekable<IntoIter<Token>>) -> Result<Function> {
    let function = match iter.peek() {
        Some(Token::Function(f)) => f.clone(),
        Some(t) => return Err(ParseError::UnexpectedToken(t.clone())),
        _ => return Err(ParseError::MissingFunction),
    };
    iter.next();

    match_token(&iter.next(), Token::Symbol(Symbol::LeftParen))?;

    let mut args: Vec<Expression> = Vec::new();

    loop {
        match iter.peek() {
            Some(Token::Symbol(Symbol::RightParen)) => break,
            _ => ()
        }
        match parse_expression(iter) {
            Ok(e) => {
                args.push(e);
                if let Some(Token::Symbol(Symbol::Comma)) = iter.peek() {
                    iter.next();
                }
            },
            Err(e) => return Err(e),
        }
    }
    iter.next();

    return Ok(Function::new(function, args)?);
}

fn match_token(value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(_) => Ok(()),
        None => return Err(ParseError::MissingToken(expect))
    }
}