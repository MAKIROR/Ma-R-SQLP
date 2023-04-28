use std::{
    rc::Rc,
    cell::RefCell,
};
use super::structs::NodeType;

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub node: NodeType,
    pub children: Vec<ASTNode>,
    parent: Option<Rc<RefCell<ASTNode>>>
}

impl ASTNode {
    pub fn new(node: NodeType, parent: Option<Rc<RefCell<ASTNode>>>) -> Self {
        Self {
            node,
            children: Vec::new(),
            parent,
        }
    }
    
    pub fn add_child(&mut self, mut node: ASTNode) {
        node.parent = Some(Rc::new(RefCell::new(self.clone())));
        self.children.push(node);
    }

    pub fn parent(&self) -> Option<Rc<RefCell<ASTNode>>> {
        self.parent.clone()
    }
} 