use std::rc::Rc;
use super::super::datatype::token::*;

struct ASTNode {
    node_type: Token,
    parent: Option<Rc<ASTNode>>,
    children: Vec<Rc<ASTNode>>,
}

impl ASTNode {
    // todo
}