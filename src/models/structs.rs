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
    Arg(Arg),
    Table(String),
    Column(String),
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
    pub ops: Vec<Symbol>,
    pub literals: Vec<String>,
}

impl Expression {
    pub fn new() -> Self {
        Self {
            ops: Vec::new(),
            literals: Vec::new(),
        }
    }
    pub fn new_with_symbol(ops: Vec<Symbol>) -> Self {
        Self {
            ops,
            literals: Vec::new(),
        }
    }
}