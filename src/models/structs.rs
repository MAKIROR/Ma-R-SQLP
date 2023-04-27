pub enum NodeType {
    SelectStatement {
        select: Select,
        from: Table,
        filter: Option<Filter>,
    },
    InsertStatement {
        table: Table,
        columns: Vec<String>,
        values: Vec<Vec<Expression>>,
    },
}

pub struct Table {
    pub name: String,
}

pub enum Expression {
    StringLiteral(String),
    NumericLiteral(i64),
    ColumnRef(ColumnRef),
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
