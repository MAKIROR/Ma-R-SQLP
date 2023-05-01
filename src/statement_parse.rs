use super::{
    error::{ParseError, Result},
    datatype::token::*,
    models::{
        ast::*,
        structs::{
            NodeType,
            Expression,
        },
    },
    datatype::{
        keyword::Keyword,
        symbol::Symbol,
    },
};

pub fn parse_select(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter();
    let mut column_names = Vec::new();   
    let mut node = ASTNode::new(NodeType::Select, None);

    match_token(&iter.next(), Token::Keyword(Keyword::Select))?;

    loop {
        match iter.next() {
            Some(Token::Identifier(name)) => column_names.push(name),
            Some(Token::Symbol(_)) => continue,
            Some(Token::Keyword(Keyword::From)) => break,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingToken(Token::Keyword(Keyword::From))) 
        }
    }

    match iter.next() {
        Some(Token::Identifier(name)) => {
            node.add_child(ASTNode::new(NodeType::Table(name), None));
        },
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

pub fn parse_insert(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();
    let mut column_names = Vec::new();   
    let mut node = ASTNode::new(NodeType::Insert, None);

    match_token(&iter.next(), Token::Keyword(Keyword::Insert))?;
    match_token(&iter.next(), Token::Keyword(Keyword::Into))?;

    match iter.next() {
        Some(Token::Identifier(name)) => {
            node.add_child(ASTNode::new(NodeType::Table(name.clone()), None));
        },
        _ => return Err(ParseError::MissingToken(Token::Identifier("Table name".to_string())))
    }

    if let Some(Token::Symbol(Symbol::LeftParen)) = iter.peek() {
        iter.next();
        loop {
            match iter.next() {
                Some(Token::Identifier(name)) => column_names.push(name.clone()),
                Some(Token::Symbol(Symbol::RightParen)) => break,
                Some(Token::Symbol(_)) => continue,
                Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
                None => return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen)))
            }
        }
    }

    match_token(&iter.next(), Token::Keyword(Keyword::Values))?;
    match_token(&iter.next(), Token::Symbol(Symbol::LeftParen))?;

    let mut values = Vec::new();
    loop {
        match iter.next() {
            Some(Token::Identifier(value)) | Some(Token::Num(value)) => values.push(value),
            Some(Token::Symbol(Symbol::RightParen)) => break,
            Some(Token::Symbol(_)) => continue,
            Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
            None => return Err(ParseError::MissingToken(Token::Symbol(Symbol::RightParen)))
        }
    }

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
        children.push(ASTNode::new(child_type, None));
    }

    node.set_child(children);
    Ok(node)
}

pub fn parse_delete(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter();
    let mut node = ASTNode::new(NodeType::Delete, None);

    match_token(&iter.next(), Token::Keyword(Keyword::Delete))?;
    match_token(&iter.next(), Token::Keyword(Keyword::Into))?;

    match iter.next() {
        Some(Token::Identifier(name)) => {
            node.add_child(ASTNode::new(NodeType::Table(name.clone()), None));
        },
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

fn parse_condition(iter: &mut std::vec::IntoIter<Token>) -> Result<Option<ASTNode>> {
    let mut root = ASTNode::new(NodeType::Condition(Expression::new()), None);

    match iter.next() {
        Some(Token::Keyword(Keyword::Where)) => (),
        None
        | Some(Token::Symbol(Symbol::Semicolon))
        | Some(Token::Symbol(Symbol::Comma))  => return Ok(None),
        Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
    }

    let mut current_node = &mut root;
    
    while let Some(token) = iter.next() {
        match token {
            Token::Keyword(_) => {
                return Err(ParseError::UnexpectedToken(token.clone()));
            }
            Token::Symbol(sym) => {
                match sym {
                    Symbol::LeftParen => {
                        let node = ASTNode::new(NodeType::Condition(Expression::new()), None);
                        current_node.add_child(node);
                        current_node = current_node.children.last_mut().unwrap();
                    },
                    Symbol::RightParen => {
                        if let None = current_node.parent() {
                            return Err(ParseError::SyntaxError("unmatched right parenthesis.".to_string()));
                        }
                    },
                    Symbol::Comma 
                    | Symbol::Dot
                    | Symbol::Plus
                    | Symbol::Minus
                    | Symbol::Slash
                    | Symbol::Percent => return Err(ParseError::UnexpectedToken(Token::Symbol(sym.clone()))),
                    s => {
                        let node = ASTNode::new(
                            NodeType::Condition(
                                Expression::new_with_symbol(
                                    vec![s]
                                )
                            ), 
                            None
                        );
                        current_node.add_child(node);
                        current_node = current_node.children.last_mut().unwrap();
                    },
                }
            }
            Token::Identifier(s) => {
                match current_node.node {
                    NodeType::Condition(_) => {
                        current_node.add_child(ASTNode::new(NodeType::Column(s), None));
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
    Ok(Some(root))
}

fn match_token(mut value: &Option<Token>, expect: Token) -> Result<()> {
    return match value {
        Some(token) => Ok(()),
        None => return Err(ParseError::MissingToken(expect))
    }
}