use super::{
    super::datatype::{
        symbol::Symbol,
        function::FunctionT
    },
    structs::{Statement, Expression},
};

#[derive(Debug, Clone)]
pub enum NodeType {
    Statement(Box<Statement>),
    Values,
    Symbol(Symbol),
    Value(Value),
    Function(Box<Function>),
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

    pub fn new_node(node: NodeType) -> Self {
        Self::new(node, None, None)
    }
    
    pub fn set_left(&mut self, node: ASTNode) {
        self.left = Some(Box::new(node));
    }

    pub fn set_right(&mut self, node: ASTNode) {
        self.right = Some(Box::new(node));
    }
} 

#[derive(Debug, Clone)]
pub enum Value {
    Identifier(String),
    Number(String),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum Function {
    Sum(Expression),
    Avg(Expression),
    Count(Expression),
    Max(Expression),
    Min(Expression),
    Concat(Vec<Expression>),
}

impl Function {
    pub fn new(func: FunctionT, args: Vec<Expression>) -> Self {
        match func {
            FunctionT::Sum => Self::Sum(args[0].clone()),
            FunctionT::Avg => Self::Avg(args[0].clone()),
            FunctionT::Count => Self::Count(args[0].clone()),
            FunctionT::Max => Self::Max(args[0].clone()),
            FunctionT::Min => Self::Min(args[0].clone()),
            FunctionT::Concat => Self::Concat(args.clone()),
        }
    }
}