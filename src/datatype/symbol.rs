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

impl Symbol {
    pub fn is_operator(&self) -> bool {
        match self {
            Symbol::Comma
            | Symbol::Dot
            | Symbol::Asterisk
            | Symbol::Plus
            | Symbol::Minus
            | Symbol::Slash
            | Symbol::Percent
            | Symbol::LeftParen
            | Symbol::RightParen => true,
            _ => false,
        }
    }

    pub fn is_comparator(&self) -> bool {
        match self {
            Symbol::Equal
            | Symbol::NotEqual
            | Symbol::LessThan
            | Symbol::GreaterThan
            | Symbol::LessThanOrEqual
            | Symbol::GreaterThanOrEqual => true,
            _ => false,
        }
    }
    
    pub fn get_priority(&self) -> i32 {
        match self {
            Symbol::Plus | Symbol::Minus => 1,
            Symbol::Dot | Symbol::Slash => 2,
            _ => 0,
        }
    }
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