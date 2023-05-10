use std::fmt;

#[derive(Debug, PartialEq, Clone)]
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
    Into,
    InnerJoin,
    LeftJoin,
    RightJoin,
    FullJoin,
    Values,
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
    let mut iter = s.split_whitespace();
    let first = iter.next()?.to_uppercase();

    match first.as_str() {
        "SELECT" => Some(Keyword::Select),
        "INSERT" => Some(Keyword::Insert),
        "UPDATE" => Some(Keyword::Update),
        "DELETE" => Some(Keyword::Delete),
        "FROM" => Some(Keyword::From),
        "WHERE" => Some(Keyword::Where),
        "GROUP" => {
            if let Some(next) = iter.next() {
                if next == "BY" {
                    return Some(Keyword::GroupBy);
                }
            }
            None
        }
        "ORDER" => Some(Keyword::OrderBy),
        "JOIN" => Some(Keyword::Join),
        "INTO" => Some(Keyword::Into),
        "INNER" => {
            if iter.next() == Some("JOIN") {
                return Some(Keyword::InnerJoin);
            }
            None
        }
        "LEFT" => {
            if iter.next() == Some("JOIN") {
                return Some(Keyword::LeftJoin);
            } else if iter.next() == Some("OUTER") && iter.next() == Some("JOIN") {
                return Some(Keyword::LeftJoin);
            }
            None
        }
        "RIGHT" => {
            if iter.next() == Some("JOIN") {
                return  Some(Keyword::RightJoin);
            }
            None
        }
        "FULL" => {
            if iter.next() == Some("JOIN") {
                return Some(Keyword::FullJoin);
            }
            None
        }
        "VALUES" => Some(Keyword::Values),
        "ON" => Some(Keyword::On),
        "AS" => Some(Keyword::As),
        "DISTINCT" => Some(Keyword::Distinct),
        "ALL" => Some(Keyword::All),
        "EXISTS" => Some(Keyword::Exists),
        "HAVING" => Some(Keyword::Having),
        "UNION" => Some(Keyword::Union),
        "NOT" => Some(Keyword::Not),
        "AND" => Some(Keyword::And),
        "OR" => Some(Keyword::Or),
        _ => None,
    }
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keyword::Select => write!(f, "SELECT"),
            Keyword::Insert => write!(f, "INSERT"),
            Keyword::Update => write!(f, "UPDATE"),
            Keyword::Delete => write!(f, "DELETE"),
            Keyword::From => write!(f, "FROM"),
            Keyword::Where => write!(f, "WHERE"),
            Keyword::GroupBy => write!(f, "GROUP BY"),
            Keyword::OrderBy => write!(f, "ORDER BY"),
            Keyword::Join => write!(f, "JOIN"),
            Keyword::Into => write!(f, "INTO"),
            Keyword::InnerJoin => write!(f, "INNER JOIN"),
            Keyword::LeftJoin => write!(f, "LEFT JOIN"),
            Keyword::RightJoin => write!(f, "RIGHT JOIN"),
            Keyword::FullJoin => write!(f, "FULL JOIN"),
            Keyword::Values => write!(f, "VALUES"),
            Keyword::On => write!(f, "ON"),
            Keyword::As => write!(f, "AS"),
            Keyword::Distinct => write!(f, "DISTINCT"),
            Keyword::All => write!(f, "ALL"),
            Keyword::Exists => write!(f, "EXISTS"),
            Keyword::Having => write!(f, "HAVING"),
            Keyword::Union => write!(f, "UNION"),
            Keyword::Not => write!(f, "NOT"),
            Keyword::And => write!(f, "AND"),
            Keyword::Or => write!(f, "OR"),
        }
    }
}

pub trait KeywordExt {
    fn has_suffix(&self) -> bool;
}

impl KeywordExt for String {
    fn has_suffix(&self) -> bool {
        match self.to_uppercase().as_str() {
            "GROUP"
            | "INNER"
            | "LEFT"
            | "OUTER"
            | "RIGHT"
            | "FULL" => true,
            _ => false,
        }
    }
}