#[derive(Debug, PartialEq)]
enum TokenType {
    Keyword(String),
    Identifier(String),
    Num(String),
    Comment(String),
    Symbol(String),
}

fn is_keyword(text: &str) -> bool {
    ["SELECT", "FROM", "WHERE", "AND", "OR", "NOT"].contains(&text)
}

fn is_symbol(text: char) -> bool {
    [',', '(', ')', '=', '<', '>'].contains(&text)
}