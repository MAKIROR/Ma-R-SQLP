use super::keyword::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Distinct,
    All,
    Values,
    As,
    Not,
    And,
    Or,
}

impl From<&Keyword> for Option<Arg> {
    fn from(keyword: &Keyword) -> Option<Arg> {
        match keyword {
            Keyword::Distinct => Some(Arg::Distinct),
            Keyword::All => Some(Arg::All),
            Keyword::Values => Some(Arg::Values),
            Keyword::As => Some(Arg::As),
            Keyword::Not => Some(Arg::Not),
            Keyword::And => Some(Arg::And),
            Keyword::Or => Some(Arg::Or),
            _ => None,
        }
    }
}