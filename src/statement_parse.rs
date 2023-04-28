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