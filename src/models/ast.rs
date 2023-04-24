pub enum NodeType {
    SelectStatement,
    FromClause,
    WhereClause,
    GroupByClause,
    HavingClause,
    OrderByClause,
    EqualityExpression,
    Identifier,
    Literal,
}

struct ASTNode {
    node_type: String,
    children: Vec<Box<ASTNode>>,
}