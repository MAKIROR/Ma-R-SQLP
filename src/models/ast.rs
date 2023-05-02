use super::{
    structs::NodeType,
    super::datatype::arg::Arg
};

#[derive(Debug, Clone)]
pub struct ASTNode {
    pub node: NodeType,
    pub children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(node: NodeType) -> Self {
        Self {
            node,
            children: Vec::new(),
        }
    }
    
    pub fn add_child(&mut self, mut node: ASTNode) {
        self.children.push(node);
    }

    pub fn new_child(&mut self, mut node: NodeType) {
        self.add_child(ASTNode::new(node));
    }

    pub fn set_child(&mut self, node: Vec<ASTNode>) {
        self.children = node;
    }

    pub fn add_arg(&mut self, arg: Arg) {
        self.new_child(NodeType::Arg(arg));
    }

    pub fn new_arg(arg: Arg) -> Self {
        ASTNode::new(NodeType::Arg(arg))
    }
} 