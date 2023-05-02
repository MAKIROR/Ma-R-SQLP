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
    Condition(Expression),
    Column(String),
    Value(String),
    ColumnValue(String, String)
}

pub enum Statement {
    SelectStatement {
        select: Select,
        from: String,
        filter: Option<Filter>,
    },
    InsertStatement {
        table: String,
        columns: Vec<String>,
        values: Vec<Vec<Expression>>,
    },
}

pub struct Select {
    pub distinct: bool,
    pub projections: Vec<Projection>,
}

pub enum Projection {
    AllColumns,
    ColumnName(String),
}

pub struct Filter {
    pub conditions: Vec<Condition>,
}

pub enum Condition {
    Comparison {
        left: ColumnRef,
        operator: String,
        right: Expression,
    }
}

pub enum ColumnRef {
    Named(String),
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