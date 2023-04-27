use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    Comma,
    Dot,
    Asterisk,
    Plus,
    Minus,
    Slash,
    Percent,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    LeftParen,
    RightParen,
    Semicolon,
}

pub fn parse_symbol(s: &str) -> Option<Symbol> {
    match s {
        "," => Some(Symbol::Comma),
        "." => Some(Symbol::Dot),
        "*" => Some(Symbol::Asterisk),
        "+" => Some(Symbol::Plus),
        "-" => Some(Symbol::Minus),
        "/" => Some(Symbol::Slash),
        "%" => Some(Symbol::Percent),
        "=" => Some(Symbol::Equal),
        "!=" => Some(Symbol::NotEqual),
        "<" => Some(Symbol::LessThan),
        ">" => Some(Symbol::GreaterThan),
        "<=" => Some(Symbol::LessThanOrEqual),
        ">=" => Some(Symbol::GreaterThanOrEqual),
        "(" => Some(Symbol::LeftParen),
        ")" => Some(Symbol::RightParen),
        ";" => Some(Symbol::Semicolon),
        _ => None,
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Symbol::Comma => write!(f, ","),
            Symbol::Dot => write!(f, "."),
            Symbol::Asterisk => write!(f, "*"),
            Symbol::Plus => write!(f, "+"),
            Symbol::Minus => write!(f, "-"),
            Symbol::Slash => write!(f, "/"),
            Symbol::Percent => write!(f, "%"),
            Symbol::Equal => write!(f, "="),
            Symbol::NotEqual => write!(f, "!="),
            Symbol::LessThan => write!(f, "<"),
            Symbol::GreaterThan => write!(f, ">"),
            Symbol::LessThanOrEqual => write!(f, "<="),
            Symbol::GreaterThanOrEqual => write!(f, ">="),
            Symbol::LeftParen => write!(f, "("),
            Symbol::RightParen => write!(f, ")"),
            Symbol::Semicolon => write!(f, ";"),
        }
    }
}