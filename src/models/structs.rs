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

struct Table {
    pub name: String,
}

enum Expression {
    StringLiteral(String),
    NumericLiteral(i64),
    ColumnRef(ColumnRef),
}

struct Select {
    pub distinct: bool,
    pub projections: Vec<Projection>,
}

enum Projection {
    AllColumns,
    ColumnName(String),
}

struct Filter {
    pub conditions: Vec<Condition>,
}

enum Condition {
    Comparison {
        left: ColumnRef,
        operator: String,
        right: Expression,
    }
}

enum ColumnRef {
    Named(String),
}
