use super::super::datatype::{
    symbol::Symbol,
    arg::Arg,
};

#[derive(Debug, Clone)]
pub enum NodeType {
    Select,
    Insert,
    Delete,
    Update,
    Values,
    Arg(Arg),
    Symbol(Symbol),
    Identifier(String),
    Number(String),
    Value(String),
    ColumnValue(String, String)
}

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub node: NodeType,
    pub left: Option<Box<ASTNode>>,
    pub right: Option<Box<ASTNode>>,
}

impl ASTNode {
    pub fn default(node: NodeType) -> Self {
        Self {
            node,
            left: None,
            right: None,
        }
    }

    pub fn new(
        node: NodeType,
        left: Option<Box<ASTNode>>,
        right: Option<Box<ASTNode>>
    ) -> Self {
        Self {
            node,
            left,
            right,
        }
    }
    
    pub fn set_left(&mut self, node: ASTNode) {
        self.left = Some(Box::new(node));
    }

    pub fn new_left(&mut self, node: NodeType) {
        self.left = Some(Box::new(ASTNode::default(node)));
    }

    pub fn set_right(&mut self, node: ASTNode) {
        self.right = Some(Box::new(node));
    }

    pub fn new_right(&mut self, node: NodeType) {
        self.right = Some(Box::new(ASTNode::default(node)));
    }
} 