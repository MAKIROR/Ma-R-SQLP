use std::{
    rc::{Rc,Weak},
    cell::RefCell
};
use super::super::datatype::token::*;

struct ASTNode {
    node: Token,
    children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(node: Token) -> Self {
        Self {
            node,
            children: Vec::new()
        }
    }
    fn add_child(&mut self, node: ASTNode) {
        self.children.push(node);
    }
} 