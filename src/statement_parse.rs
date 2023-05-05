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
    let mut conditions_stack: Vec<Condition> = Vec::new();
    
    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(Keyword::All) => {
                // TODO:
            },
            Token::Keyword(Keyword::Or) => {
                // TODO:
            },
            Token::Keyword(Keyword::Not) => {
                // TODO:
            },
            Token::Keyword(_) => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            }
            Token::Symbol(sym) => {
                // TODO:
            }
            Token::Identifier(s) => {
                // TODO:
            }
            Token::Num(n) => {
                // TODO:
            }
            Token::Comment(_) => (),
        }
    }

    return Ok(Some(Filter { conditions }));
}

fn parse_next_expression(iter: &mut Peekable<IntoIter<Token>>) -> Result<Expression> {
    let mut left_node: Option<Expression> = None;
    let mut operator: Option<Symbol> = None;

    while let Some(token) = iter.peek() {
        if left_node.is_some() && operator.is_none() && !token.is_operator() {
            break;
        }
        match token {
            Token::Num(ref s) | Token::Identifier(ref s) => {
                if let Some(op) = operator {
                    let right_node = ASTNode::default(NodeType::Identifier(s.clone()));
                    left_node = Some(Expression::new(
                        left_node.take().unwrap().ast, 
                        op,
                        right_node
                    ));
                    operator = None;
                } else {
                    left_node = Some(Expression::new_with_ast(ASTNode::default(NodeType::Identifier(s.clone()))));
                }
            }
            Token::Symbol(Symbol::LeftParen) => {
                let next_expr = parse_next_expression(iter)?;
                if let Some(op) = operator {
                    let right_node = next_expr;
                    left_node = Some(Expression::new(
                        left_node.take().unwrap().ast,
                        op,
                        right_node.ast
                    ));
                    operator = None;
                } else {
                    left_node = Some(next_expr);
                }
                match iter.next() {
                    Some(Token::Symbol(Symbol::RightParen)) => (),
                    Some(t) => return Err(ParseError::UnexpectedToken(t)),
                    None => return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen))),
                }
            }
            Token::Symbol(ref s) => {
                if !s.is_operator() || operator.is_some() {
                    return Err(ParseError::UnexpectedToken(token.clone()));
                }
                operator = Some(s.clone());
            }
            Token::Keyword(k) => {
                if let Some(n) = left_node {
                    if operator.is_none() {
                        return Ok(n);
                    }
                }
                return Err(ParseError::IncorrectExpression);
            }
            _ => return Err(ParseError::UnexpectedToken(token.clone())),
        }
    }

    iter.next();

    if let Some(n) = left_node {
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