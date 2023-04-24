#[derive(Debug, PartialEq)]
pub enum Keyword {
    Select,
    Insert,
    Update,
    Delete,
    From,
    Where,
    GroupBy,
    OrderBy,
    Join,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
    On,
    As,
    Distinct,
    All,
    Exists,
    Having,
    Union,
    Not,
    And,
    Or,
}

pub fn parse_keyword(s: &str) -> Option<Keyword> {
    let upper_case = s.to_uppercase();
    match upper_case.as_str() {
        "SELECT" => Some(Keyword::Select),
        "INSERT" => Some(Keyword::Insert),
        "UPDATE" => Some(Keyword::Update),
        "DELETE" => Some(Keyword::Delete),
        "FROM" => Some(Keyword::From),
        "WHERE" => Some(Keyword::Where),
        "GROUP" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                match next.to_uppercase().as_str() {
                    "BY" => {
                        return Some(Keyword::GroupBy);
                    }
                    _ => {}
                }
            }
            None
        }
        "ORDER" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                match next.to_uppercase().as_str() {
                    "BY" => {
                        return Some(Keyword::OrderBy);
                    }
                    _ => {}
                }
            }
            None
        }
        "JOIN" => Some(Keyword::Join),
        "INNER" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                if next.to_uppercase() == "JOIN" {
                    return Some(Keyword::InnerJoin);
                }
            }
            None
        }
        "LEFT" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                match next.to_uppercase().as_str() {
                    "JOIN" => {
                        return Some(Keyword::LeftJoin);
                    }
                    "OUTER" => {
                        if let Some(tail) = s.split_whitespace().nth(2) {
                            if tail.to_uppercase() == "JOIN" {
                                return Some(Keyword::LeftJoin);
                            }
                        }
                        None
                    }
                    _ => None,
                }
            } else {
                None
            }
        }
        "RIGHT" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                if next.to_uppercase() == "JOIN" {
                    return Some(Keyword::RightJoin);
                }
            }
            None
        }
        "FULL" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                if next.to_uppercase() == "JOIN" {
                    return Some(Keyword::FullJoin);
                }
            }
            None
        }
        "ON" => Some(Keyword::On),
        "AS" => Some(Keyword::As),
        "DISTINCT" => Some(Keyword::Distinct),
        "ALL" => Some(Keyword::All),
        "EXISTS" => Some(Keyword::Exists),
        "HAVING" => Some(Keyword::Having),
        "UNION" => Some(Keyword::Union),
        "NOT" => Some(Keyword::Not),
        "AND" => {
            if let Some(next) = s.split_whitespace().nth(1) {
                if next.to_uppercase() == "NOT" {
                    return Some(Keyword::And);
                }
            }
            None
        }
        "OR" => Some(Keyword::Or),
        _ => None,
    }
}