use super::structs::NodeType;

pub struct ASTNode {
    node: NodeType,
    children: Vec<ASTNode>,
}

impl ASTNode {
    pub fn new(node: NodeType) -> Self {
        Self {
            node,
            children: Vec::new()
        }
    }
    
    pub fn add_child(&mut self, node: ASTNode) {
        self.children.push(node);
    }
} 