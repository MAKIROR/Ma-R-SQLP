use super::super::datatype::{
    keyword::Keyword,
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
    Symbol(Symbol),
    Identifier(String),
    Value(String),
    ColumnValue(String, String)
}

pub enum Statement {
    SelectStatement {
        distinct: bool,
        projections: Projection,
        from: String,
        filter: Option<Filter>,
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

pub struct Filter {
    pub conditions: Vec<Condition>,
}

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
    pub fn new(s: Symbol) -> Self {
        Self {
            ast: ASTNode::new(NodeType::Symbol(s))
        }
    }

    pub fn add_child(&mut self, mut node: ASTNode) {
        self.ast.children.push(node);
    }
}