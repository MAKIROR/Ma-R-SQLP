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
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();
    let mut node = ASTNode::new(NodeType::Select);

    match_token(&iter.next(), Token::Keyword(Keyword::Select))?;
    let distinct = parse_optional_args_or(&mut iter, vec![Arg::All, Arg::Distinct], Arg::All);
    let projection = parse_projection(&mut iter)?;
    let from = parse_table(&mut iter)?;
    let filter = match parse_condition(&mut iter)?;
    
    Ok(node)
}

pub fn parse_insert(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();
    let mut node = ASTNode::new(NodeType::Insert);

    match_token(&iter.next(), Token::Keyword(Keyword::Insert))?;
    match_token(&iter.next(), Token::Keyword(Keyword::Into))?;

    match iter.next() {
        Some(Token::Identifier(name)) => node.new_child(NodeType::Table(name.clone())),
        _ => return Err(ParseError::MissingToken(Token::Identifier("Table name".to_string())))
    }

    let column_names = parse_column(&mut iter)?;
    match_token(&iter.next(), Token::Keyword(Keyword::Values))?;
    let values = parse_column(&mut iter)?;

    if !column_names.is_empty() && column_names.len() != values.len() {
        return Err(ParseError::IncorrectValueCount(column_names.len()));
    }

    let mut children = Vec::new();
    for (i, value) in values.iter().enumerate() {
        let child_type = if column_names.is_empty() {
            NodeType::Value(value.clone())
        } else {
            NodeType::ColumnValue(column_names[i].clone(), value.clone())
        };
        children.push(ASTNode::new(child_type));
    }

    node.set_child(children);
    Ok(node)
}

pub fn parse_delete(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();
    let mut node = ASTNode::new(NodeType::Delete);

    match_token(&iter.next(), Token::Keyword(Keyword::Delete))?;
    match_token(&iter.next(), Token::Keyword(Keyword::Into))?;

    match iter.next() {
        Some(Token::Identifier(name)) => node.new_child(NodeType::Table(name.clone())),
        _ => return Err(ParseError::MissingToken(Token::Identifier("Table name".to_string())))
    }

    match parse_condition(&mut iter)? {
        Some(c) => {
            node.add_child(c);
        },
        None => ()
    };

    Ok(node)
}

fn parse_conditions(iter: &mut Peekable<IntoIter<Token>>) -> Result<Option<Filter>> {
    match iter.next() {
        Some(Token::Keyword(Keyword::Where)) => (),
        None
        | Some(Token::Symbol(Symbol::Semicolon))
        | Some(Token::Symbol(Symbol::Comma))  => return Ok(None),
        Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
    }

    let mut paren = VecDeque::new();
    let mut conditions = Vec::new();
    
    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(_) => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            }
            Token::Symbol(sym) => {
                match sym {
                    Symbol::LeftParen => {
                        let condition = Expression::new();
                        conditions.push(condition)
                        paren.push_back(Symbol::LeftParen);
                    },
                    Symbol::RightParen => {
                        if Some(Symbol::LeftParen) != paren.pop_back() {
                            return Err(ParseError::UnexpectedToken(Token::Symbol(Symbol::RightParen)));
                        }
                    },
                    Symbol::Comma 
                    | Symbol::Dot
                    | Symbol::Plus
                    | Symbol::Minus
                    | Symbol::Slash
                    | Symbol::Percent => return Err(ParseError::UnexpectedToken(Token::Symbol(sym.clone()))),
                    s => {
                        let expression = Expression::new_with_symbol(vec![s]);
                    },
                }
            }
            Token::Identifier(s) => {
                match current_node.node {
                    NodeType::Condition(_) => {
                        current_node.new_child(NodeType::Column(s));
                    }
                    _ => {
                        println!("{:?}", current_node);
                        return Err(ParseError::UnexpectedToken(Token::Identifier(s.clone())));
                    }
                }
            }
            Token::Num(n) => {
                if let NodeType::Condition(ref mut expr) = current_node.node {
                    expr.literals.push(n);
                } else {
                    return Err(ParseError::UnexpectedToken(Token::Num(n.clone())));
                }
            }
            Token::Comment(_) => (),
        }
    }
}

fn parse_condition(iter: &mut Peekable<IntoIter<Token>>) -> Result<Option<Condition>> {

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
