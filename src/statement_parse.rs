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
    let mut tokens = t.clone();
    let mut iter = tokens.into_iter();
    let mut column_names = Vec::new();   
    let mut node = ASTNode::new(NodeType::Select, None);

    iter.next();
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
    
    match iter.next() {
        Some(Token::Keyword(Keyword::Where)) => {
            let condition = parse_condition(&mut iter)?;
            node.add_child(condition);
        },
        Some(token) => return Err(ParseError::UnexpectedToken(token.clone())),
        None => (),
    }
    Ok(node)
}

pub fn parse_insert(t: &Vec<Token>) -> Result<ASTNode> {
    let tokens = t.clone();
    let mut iter = tokens.into_iter().peekable();
    let mut column_names = Vec::new();   
    let mut node = ASTNode::new(NodeType::Insert, None);

    iter.next();
    match iter.next() {
        Some(Token::Keyword(Keyword::Into)) => (),
        _ => return Err(ParseError::MissingToken(Token::Keyword(Keyword::Into)))
    }

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

    match iter.next() {
        Some(Token::Keyword(Keyword::Values)) => (),
        _ => return Err(ParseError::MissingToken(Token::Keyword(Keyword::Values)))
    }

    match iter.next() {
        Some(Token::Symbol(Symbol::LeftParen)) => (),
        _ => return Err(ParseError::MissingToken(Token::Symbol(Symbol::LeftParen)))
    }

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

fn parse_condition(iter: &mut std::vec::IntoIter<Token>) -> Result<ASTNode> {
    let mut root = ASTNode::new(NodeType::Condition(Expression::new()), None);

    let mut current_node = &mut root;
    let mut current_expr = &mut current_node.node;
    
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
                        current_expr = &mut current_node.node;
                    },
                    Symbol::RightParen => {
                        if let Some(parent_node) = current_node.parent() {
                            let mut parent_ast = parent_node.borrow_mut();
                            let mut current_node = &mut parent_ast;
                        } else {
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
                        current_expr = &mut current_node.node;
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
    Ok(root)
}