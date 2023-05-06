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
        arg::Arg,
    },
};

pub fn parse_select(t: &Vec<Token>) -> Result<ASTNode> {
   // TODO:
   todo!()
}

pub fn parse_insert(t: &Vec<Token>) -> Result<ASTNode> {
   todo!()
   // TODO:
}

pub fn parse_delete(t: &Vec<Token>) -> Result<ASTNode> {
   todo!()
   // TODO:
}

fn parse_conditions(iter: &mut Peekable<IntoIter<Token>>) -> Result<Option<Filter>> {
    match iter.next() {
        Some(Token::Keyword(Keyword::Where)) => (),
        _  => return Ok(None),
    }

    let mut conditions = Vec::new();
    let mut last_condition = parse_condition(iter)?;

    conditions.push(last_condition.clone());

    while let Some(token) = iter.peek().cloned() {
        match token {
            Token::Keyword(Keyword::And) => {
                iter.next();
                let next_condition = parse_condition(iter)?;
                last_condition = Condition::And {
                    left: Box::new(last_condition.clone()),
                    right: Box::new(next_condition)
                };
                conditions.pop();
                conditions.push(last_condition.clone());
            },
            Token::Keyword(Keyword::Or) => {
                iter.next();
                let next_condition = parse_condition(iter)?;
                last_condition = Condition::Or {
                    left: Box::new(last_condition.clone()),
                    right: Box::new(next_condition)
                };
                conditions.pop();
                conditions.push(last_condition.clone());
            },
            Token::Keyword(Keyword::Not) => {
                iter.next();
                let next_condition = parse_condition(iter)?;
                last_condition = Condition::Not(Box::new(next_condition));
                conditions.pop();
                conditions.push(last_condition.clone());
            },
            Token::Symbol(Symbol::LeftParen) => {
                iter.next();
                last_condition = parse_condition(iter)?;
                if let Some(Token::Symbol(Symbol::RightParen)) = iter.next() {
                    conditions.push(last_condition.clone());
                } else {
                    return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen)));
                }
            }
            Token::Keyword(_) | Token::Symbol(_) | Token::Num(_) => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            }
            Token::Identifier(s) => {
                last_condition = parse_condition(iter)?;
                conditions.push(last_condition.clone());
            }
            Token::Comment(_) => (),
        }
    }

    return Ok(Some(Filter { conditions }));
}

fn parse_condition(iter: &mut Peekable<IntoIter<Token>>) -> Result<Condition> {
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
            Token::Symbol(Symbol::Semicolon) | Token::Keyword(_) => {
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

fn match_token(mut value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(token) => Ok(()),
        None => return Err(ParseError::MissingToken(expect))
    }
}