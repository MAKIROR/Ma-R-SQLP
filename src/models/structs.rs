use super::{
    super::datatype::symbol::Symbol,
    ast::*,
};

pub enum Statement {
    SelectStatement {
        distinct: bool,
        projections: Projection,
        from: String,
        filter: Option<Condition>,
    },
    InsertStatement {
        table: String,
        column_value: Vec<(String, String)>,
    },
}

pub enum Projection {
    AllColumns,
    Columns(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum Condition {
    And {
        left: Box<Condition>,
        right: Box<Condition>,
    },
    Or {
        left: Box<Condition>,
        right: Box<Condition>,
    },
    Not(Box<Condition>),
    Comparison {
        left: String,
        operator: Symbol,
        right: Expression,
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub ast: ASTNode,
}

impl Expression {
    pub fn new(left: ASTNode, symbol: Symbol, right: ASTNode) -> Self {
        Self { 
            ast: ASTNode::new(
                NodeType::Symbol(symbol),
                Some(Box::new(left)),
                Some(Box::new(right))
            )
        }
    }

    pub fn new_with_ast(ast: ASTNode) -> Self {
        Self { ast }
    }

    pub fn new_with_symbol(s: Symbol) -> Self {
        Self {
            ast: ASTNode::default(NodeType::Symbol(s))
        }
    }
}